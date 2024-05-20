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
    if let Err(e) = dotenv::dotenv() {
        eprintln!("Failed to read .env file: {}", e);
        std::process::exit(1);
    }

    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("JWT_SECRET must be set.");
            std::process::exit(1);
        }
    };

    let jwt_cache = Arc::clone(&JWT_CACHE);

    let api = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(move |login_request: LoginRequest| {
            let cache = jwt_cache.clone();
            let secret_clone = jwt_secret.clone();
            async move {
                authenticate_user(login_request)
                    .and_then(|user_id| {
                        if let Some(cached_token) = cache.get(&user_id) {
                            Ok(warp::reply::with_status(cached_token.value().clone(), StatusCode::OK))
                        } else {
                            create_jwt(&secret_clone, &user_id)
                                .map(|token| {
                                    cache.insert(user_id, token.clone());
                                    warp::reply::with_status(token, StatusCode::OK)
                                })
                                .map_err(|e| reject::custom(e))
                        }
                    })
                    .or_else(|e| Err(reject::custom(e)))
            }
        })
        .recover(handle_rejection);

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await.unwrap(); // Added unwrap to handle potential runtime errors gracefully.
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
    InternalServerError,
}

impl reject::Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::AuthenticationError => write!(f, "Authentication failure"),
            Error::JWTError => write!(f, "JWT processing error"),
            Error::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

fn authenticate_user(login_request: LoginRequest) -> Result<String, Error> {
    if login_request.username == "admin" && login_request.password == "password" {
        Ok("user_id_example".to_string())
    } else {
        Err(Error::AuthenticationError)
    }
}

fn create_jwt(secret: &str, user_id: &str) -> Result<String, Error> {
    let expiration = match chrono::Utc::now().checked_add_signed(chrono::Duration::seconds(60)) {
        Some(time) => time.timestamp(),
        None => return Err(Error::InternalServerError),
    };

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