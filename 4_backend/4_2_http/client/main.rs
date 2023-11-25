use std::io::{self, Write};

// use reqwest::{Client, Response};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Response {
    result: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;

        let client = Client::new();
        let res = client
            .post("http://localhost:3000/command")
            .json(&json!({"command": input}))
            .send()
            .await?;

        let result: Response = res.json().await?;
        println!("{}", result.result);
    }
}