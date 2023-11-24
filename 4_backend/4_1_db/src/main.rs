use std::io::{self, Write};

mod db;

#[cfg(test)]
mod tests;

use crate::db::*;

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

        match tokens.as_slice() {
            ["create", "user", name] => {
                println!("Creating user with name: {}", name);
                db.create_user(name).await?;
            }
            ["delete", "user", id] => {
                println!("Deleting user with ID: {}", id);
                db.delete_user(id.parse()?).await?;
            }
            ["update", "user", id, field, value] => {
                println!("Updating field {} of user with ID {}: {}", field, id, value);
                db.update_user(id.parse()?, field, value).await?;
            }
            ["assign", "role", role_slug, "to", "user", user_id] => {
                println!("Assigning role {} to user with ID {}", role_slug, user_id);
                db.assign_role(user_id.parse()?, role_slug).await?;
            }
            ["unassign", "role", role_slug, "from", "user", user_id] => {
                println!(
                    "Unassigning role {} from user with ID {}",
                    role_slug, user_id
                );
                db.unassign_role(user_id.parse()?, role_slug).await?;
            }
            ["list", "roles"] => {
                println!("Listing all roles");
                db.list_roles().await?;
            }
            ["list", "role", slug] => {
                println!("Listing role with slug: {}", slug);
                db.list_role(slug).await?;
            }
            ["list", "users"] => {
                println!("Listing all users");
                db.list_users().await?;
            }
            ["list", "user", id] => {
                println!("Listing user with ID: {}", id);
                db.list_user(id.parse()?).await?;
            }
            ["create", "role", slug, name, permissions] => {
                println!(
                    "Creating role with slug: {}, name: {}, permissions: {}",
                    slug, name, permissions
                );
                db.create_role(slug, name, permissions).await?;
            }
            ["delete", "role", slug] => {
                println!("Deleting role with slug: {}", slug);
                db.delete_role(slug).await?;
            }
            ["update", "role", slug, field, value] => {
                println!(
                    "Updating field {} of role with slug {}: {}",
                    field, slug, value
                );
                db.update_role(slug, field, value).await?;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
