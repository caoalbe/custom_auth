use axum::extract::Json;
use axum::{
    Router,
    routing::post,
};

use custom_auth::{User, write_string_to_file, where_row_match};

async fn sign_up(Json(payload): Json<User>) -> &'static str {
    write_string_to_file(payload.to_string().as_str()).unwrap();
    "User Created"
}

async fn sign_in(Json(payload): Json<User>) -> &'static str {
    // let User {user, pass} = payload;

    if where_row_match(payload.to_string().as_str()) {
        "signed in"
    } else {
        "failed sign in"
    }
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
