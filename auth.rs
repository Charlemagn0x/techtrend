use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{http::StatusCode, reject, Rejection, Reply};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

static JWT_CACHE: Lazy<Arc<DashMap<String, String>>> = Lazy::new(|| Arc::new(DashMap::new()));

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let jwt_cache = Arc::clone(&JWT_CACHE);

    let api = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(move |login_request: LoginRequest| {
            let cache = jwt_cache.clone();
            let secret_clone = jwt_secret.clone();
            async move {
                authenticate_user(login_request)
                    .map_ok(move |user_id| {
                        if let Some(cached_token) = cache.get(&user_id) {
                            return Ok(cached_token.value().clone());
                        }
                        let token = create_jwt(&secret_clone, &user_id)?;
                        cache.insert(user_id, token.clone());
                        Ok(token)
                    })
                    .map_err(|e| reject::custom(e))
            }
        })
        .recover(handle_rejection)
        .map(|jwt_result| match jwt_result {
            Ok(token) => warp::reply::with_status(token, StatusCode::OK),
            Err(_) => warp::reply::with_status("Unauthorized".to_string(), StatusCode::UNAUTHORIZED),
        });

    warp::serve(api).run(([127, 0, 0, 1], 3030));
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug)]
enum Error {
    AuthenticationError,
    JWTError,
}

impl reject::Reject for Error {}

fn authenticate_user(login_request: LoginRequest) -> Result<String, Error> {
    if login_request.username == "admin" && login_request.password == "password" {
        Ok("user_id_example".to_string())
    } else {
        Err(Error::AuthenticationError)
    }
}

fn create_jwt(secret: &str, user_id: &str) -> Result<String, Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|_| Error::JWTError)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.find::<Error>().is_some() {
        let code;
        let message;

        if err.find::<Error>().contains(&&Error::AuthenticationError) {
            code = StatusCode::UNAUTHORIZED;
            message = "Invalid username or password.";
        } else if err.find::<Error>().contains(&&Error::JWTError) {
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Failed to generate JWT.";
        } else {
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Unhandled error.";
        }

        Ok(warp::reply::with_status(message, code))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}