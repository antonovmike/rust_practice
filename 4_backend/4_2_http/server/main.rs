use std::net::SocketAddr;

use axum::{routing::post, http::StatusCode, Router, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Command {
    command: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    result: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/command", post(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Json(_cmd): Json<Command>) -> Result<Json<Response>, StatusCode> {
    // 1. Parse the command
    // 2. Execute the command
    // 3. Send back the results
    todo!()
}
