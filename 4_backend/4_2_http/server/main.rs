use std::net::SocketAddr;

use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use db;

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

async fn handler(Json(cmd): Json<Command>) -> Result<Json<Response>, StatusCode> {
    // 1. Parse the command
    // 2. Execute the command
    // 3. Send back the results

    let db = db::db::DataBase::create_database("sqlite://db.sqlite3")
        .await
        .unwrap();

    let tokens: Vec<&str> = cmd.command.split_whitespace().collect();

    todo!()
}
