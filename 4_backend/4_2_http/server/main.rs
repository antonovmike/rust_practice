use std::net::SocketAddr;

use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::db::commands::handle_command;

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
    let db = db::db::DataBase::create_database("sqlite://db.sqlite3")
        .await
        .unwrap();

    let app = Router::new()
        .route("/command", post(handler))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Json(cmd): Json<Command>) -> Result<Json<Response>, StatusCode> {
    let db = db::db::DataBase::create_database("sqlite://db.sqlite3")
        .await
        .unwrap();

    let tokens: Vec<&str> = cmd.command.split_whitespace().collect();

    let result = handle_command(&db, tokens)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(Response { result }))
    // Ok(Json(Response { result: serde_json::from_str(&result).unwrap() }))
}
