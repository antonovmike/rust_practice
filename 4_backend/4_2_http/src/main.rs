use std::io::{self, Write};

mod commands;
mod db;

#[cfg(test)]
mod tests;

use crate::commands::handle_command;
use crate::db::*;

/*
The logic for handling commands should be moved to the server daemon.
The server daemon will receive commands from the client, parse them,
execute them, and send back the results.
The client will only be responsible for sending commands and rendering the results.
*/

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = DataBase::create_database("sqlite://db.sqlite3").await?;

    let mut input = String::new();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;

        db.create_tables().await?;

        let tokens: Vec<&str> = input.split_whitespace().collect();
        
        handle_command(&db, tokens).await?;
    }
}
