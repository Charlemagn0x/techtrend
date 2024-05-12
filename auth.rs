use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let api = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(move |login_request: LoginRequest| {
            authenticate_user(login_request)
                .map_ok(move |user_id| create_jwt(&jwt_secret, &user_id))
        })
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

fn authenticate_user(login_request: LoginRequest) -> Result<String, &'static str> {
    if login_request.username == "admin" && login_request.password == "password" {
        Ok("user_id_example".to_string()) 
    } else {
        Err("Invalid username or password")
    }
}

fn create_jwt(secret: &str, user_id: &str) -> Result<String, &'static str> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| "JWT creation error")
}

fn verify_jwt(token: &str, secret: &str) -> Result<Claims, &'static str> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| "Invalid JWT")
}

fn example_protected_route_handler(jwt_token: String, jwt_secret: String) -> Result<String, &'static str> {
    if let Ok(claims) = verify_jwt(&jwt_token, &jwt_secret) {
        Ok(format!("Access granted for user: {}", claims.sub))
    } else {
        Err("Access denied")
    }
}