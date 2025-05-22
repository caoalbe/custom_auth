use axum::extract::Json;
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use axum::{Router, routing::post};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use custom_auth::{User, where_row_match, write_string_to_file};

async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    let User { user, pass , count} = payload;
    let payload = User {
        user,
        pass: encrypt_string(pass),
        count,
    };

    write_string_to_file(payload.to_string().as_str()).unwrap();
    "User Created"
}

async fn sign_in(Json(payload): Json<User>) -> impl IntoResponse {
    let User { user, pass , count} = payload;
    let payload = User {
        user,
        pass: encrypt_string(pass),
        count,
    };

    let mut headers: HeaderMap = HeaderMap::new();
    if where_row_match(payload.to_string().as_str()) {
        let mut jwt: String = match generate_jwt("test payload".to_string(), "12345") {
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
        exp: 0,
        iat: 0,
        user,
    };

    encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(encoding_key.as_ref()),
    )
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
