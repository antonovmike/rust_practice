use std::io::{self, Write};

use sqlx::sqlite::SqlitePoolOptions;

mod db;

use crate::db::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_database().await?;

    let mut input = String::new();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(DB_URL)
            .await?;

        create_tables(&pool).await?;

        let tokens: Vec<&str> = input.split_whitespace().collect();

        match tokens.as_slice() {
            ["create", "user", name] => {
                println!("Creating user with name: {}", name);
                create_user(&pool, name).await?;
            }
            ["delete", "user", id] => {
                println!("Deleting user with ID: {}", id);
                delete_user(&pool, id.parse()?).await?;
            }
            ["update", "user", id, field, value] => {
                println!("Updating field {} of user with ID {}: {}", field, id, value);
                update_user(&pool, id.parse()?, field, value).await?;
            }
            ["assign", "role", role_slug, "to", "user", user_id] => {
                println!("Assigning role {} to user with ID {}", role_slug, user_id);
                assign_role(&pool, user_id.parse()?, role_slug).await?;
            }
            ["unassign", "role", role_slug, "from", "user", user_id] => {
                println!(
                    "Unassigning role {} from user with ID {}",
                    role_slug, user_id
                );
                unassign_role(&pool, user_id.parse()?, role_slug).await?;
            }
            ["list", "roles"] => {
                println!("Listing all roles");
                list_roles(&pool).await?;
            }
            ["list", "role", slug] => {
                println!("Listing role with slug: {}", slug);
                list_role(&pool, slug).await?;
            }
            ["list", "users"] => {
                println!("Listing all users");
                list_users(&pool).await?;
            }
            ["list", "user", id] => {
                println!("Listing user with ID: {}", id);
                list_user(&pool, id.parse()?).await?;
            }
            ["create", "role", slug, name, permissions] => {
                println!(
                    "Creating role with slug: {}, name: {}, permissions: {}",
                    slug, name, permissions
                );
                create_role(&pool, slug, name, permissions).await?;
            }
            ["delete", "role", slug] => {
                println!("Deleting role with slug: {}", slug);
                delete_role(&pool, slug).await?;
            }
            ["update", "role", slug, field, value] => {
                println!(
                    "Updating field {} of role with slug {}: {}",
                    field, slug, value
                );
                update_role(&pool, slug, field, value).await?;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
