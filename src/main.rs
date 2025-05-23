use axum::extract::Json;
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use axum::{Router, routing::post};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use custom_auth::{User, increment_user, where_row_match, write_string_to_file};

async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    let User { user, pass, count } = payload;
    let payload = User {
        user,
        pass: encrypt_string(pass),
        count,
    };

    write_string_to_file(payload.to_string().as_str()).unwrap();
    "User Created"
}

async fn sign_in(Json(payload): Json<User>) -> impl IntoResponse {
    let User { user, pass, count } = payload;
    let payload = User {
        user,
        pass: encrypt_string(pass),
        count,
    };

    let mut headers: HeaderMap = HeaderMap::new();
    if where_row_match(payload.to_string().as_str()) {
        // TODO: read encoding_key from .env file
        let mut jwt: String = match generate_jwt(payload.user, "12345") {
            Ok(val) => val,
            Err(e) => {
                panic!("generate_jwt error - {}", e)
            }
        };
        jwt = format!("custom_auth={jwt}");
        headers.insert(header::SET_COOKIE, jwt.parse().unwrap());
        (headers, "signed in")
    } else {
        (headers, "failed sign in")
    }
}

async fn increment_count(jar: CookieJar) -> impl IntoResponse {
    let jwt = match jar.get("custom_auth") {
        Some(cookie) => cookie.value(),
        None => {
            return "no cookie found".to_string();
        }
    };

    // decode the cookie and increment
    let Claims { user, .. } = decode_jwt(jwt.to_string(), "12345");
    increment_user(&user);

    return "incremented!".to_string();
}

fn encrypt_string(data: String) -> String {
    let data: &[u8] = data.as_bytes(); // convert to bytes
    let data: Vec<u8> = Sha256::digest(data)[..].to_vec(); // hash with sha256
    data.iter().map(|b: &u8| format!("{:02X}", b)).collect() // convert to hexadecimal representation
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    user: String,
}

fn generate_jwt(user: String, encoding_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(jsonwebtoken::Algorithm::HS512);

    let my_claims = Claims {
        exp: 1779457658,
        iat: 1779457658,
        user,
    };

    encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(encoding_key.as_ref()),
    )
}

fn decode_jwt(jwt_token: String, encoding_key: &str) -> Claims {
    let output = decode::<Claims>(
        &jwt_token,
        &DecodingKey::from_secret(encoding_key.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    );

    match output {
        Ok(token_data) => {
            return token_data.claims;
        }
        Err(e) => {
            panic!("decoding error: {}", e)
        }
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up))
        .route("/increment", post(increment_count));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
