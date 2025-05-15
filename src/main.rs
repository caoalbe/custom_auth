use axum::extract::Json;
use axum::{Router, routing::post};
use sha2::{Digest, Sha256};

use custom_auth::{User, where_row_match, write_string_to_file};

async fn sign_up(Json(payload): Json<User>) -> &'static str {
    let User { user, pass } = payload;
    let payload = User {
        user,
        pass: encrypt_string(pass),
    };

    write_string_to_file(payload.to_string().as_str()).unwrap();
    "User Created"
}

async fn sign_in(Json(payload): Json<User>) -> &'static str {
    let User { user, pass } = payload;
    let payload = User {
        user,
        pass: encrypt_string(pass),
    };

    if where_row_match(payload.to_string().as_str()) {
        "signed in"
    } else {
        "failed sign in"
    }
}

fn encrypt_string(data: String) -> String {
    let data: &[u8] = data.as_bytes(); // convert to bytes
    let data: Vec<u8> = Sha256::digest(data)[..].to_vec(); // hash with sha256
    data.iter().map(|b: &u8| format!("{:02X}", b)).collect() // convert to hexadecimal representation
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
