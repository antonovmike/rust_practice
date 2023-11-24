use std::io::{self, Write};

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://db.sqlite3";

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct Role {
    slug: String,
    name: String,
    permissions: String,
}

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
            .connect(&format!("{}", DB_URL))
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

async fn create_database() -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created successfully"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await?;

    Ok(pool)
}

async fn create_tables(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT
        );
        CREATE TABLE IF NOT EXISTS roles (
            slug TEXT PRIMARY KEY,
            name TEXT,
            permissions TEXT
        );
        CREATE TABLE IF NOT EXISTS users_roles (
            user_id INTEGER REFERENCES users(id),
            role_slug TEXT REFERENCES roles(slug),
            PRIMARY KEY (user_id, role_slug)
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_user(pool: &Pool<Sqlite>, name: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO users (name)
        VALUES (?)
        "#,
    )
    .bind(name)
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_role(
    pool: &Pool<Sqlite>,
    slug: &str,
    name: &str,
    permissions: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO roles (slug, name, permissions)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(slug)
    .bind(name)
    .bind(permissions)
    .execute(pool)
    .await?;

    Ok(())
}

async fn delete_user(pool: &Pool<Sqlite>, id: i32) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn delete_role(pool: &Pool<Sqlite>, slug: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        DELETE FROM roles
        WHERE slug = ?
        "#,
    )
    .bind(slug)
    .execute(pool)
    .await?;

    Ok(())
}

async fn update_user(pool: &Pool<Sqlite>, id: i32, field: &str, value: &str) -> anyhow::Result<()> {
    let query = format!("UPDATE users SET {} = ? WHERE id = ?", field);

    sqlx::query(&query)
        .bind(value)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update_role(
    pool: &Pool<Sqlite>,
    slug: &str,
    field: &str,
    value: &str,
) -> anyhow::Result<()> {
    let query = format!("UPDATE roles SET {} = ? WHERE slug = ?", field);

    sqlx::query(&query)
        .bind(value)
        .bind(slug)
        .execute(pool)
        .await?;

    Ok(())
}

async fn assign_role(pool: &Pool<Sqlite>, user_id: i32, role_slug: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO users_roles (user_id, role_slug)
        VALUES (?, ?)
        "#,
    )
    .bind(user_id)
    .bind(role_slug)
    .execute(pool)
    .await?;

    Ok(())
}

async fn unassign_role(pool: &Pool<Sqlite>, user_id: i32, role_slug: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        DELETE FROM users_roles
        WHERE user_id = ? AND role_slug = ?
        "#,
    )
    .bind(user_id)
    .bind(role_slug)
    .execute(pool)
    .await?;

    Ok(())
}

async fn list_roles(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let rows = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        "#,
    )
    .fetch_all(pool)
    .await?;

    for role in rows {
        println!("{:?}", role);
    }

    Ok(())
}
async fn list_role(pool: &Pool<Sqlite>, slug: &str) -> anyhow::Result<()> {
    let role = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        WHERE slug = ?
        "#,
    )
    .bind(slug)
    .fetch_one(pool)
    .await?;

    println!("{:?}", role);

    Ok(())
}

async fn list_users(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let rows = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        "#,
    )
    .fetch_all(pool)
    .await?;

    for user in rows {
        println!("{:?}", user);
    }

    Ok(())
}

async fn list_user(pool: &Pool<Sqlite>, id: i32) -> anyhow::Result<()> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    println!("{:?}", user);

    let roles = sqlx::query(
        r#"
        SELECT roles.slug, roles.name, roles.permissions
        FROM roles
        JOIN users_roles ON roles.slug = users_roles.role_slug
        WHERE users_roles.user_id = ?
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    for role in roles {
        let slug: String = role.get("slug");
        let name: String = role.get("name");
        let permissions: String = role.get("permissions");
        println!(
            "Role: slug={}, name={}, permissions={}",
            slug, name, permissions
        );
    }

    Ok(())
}
