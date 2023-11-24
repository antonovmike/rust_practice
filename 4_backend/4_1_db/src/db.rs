use sqlx::{migrate::MigrateDatabase, Pool, Row, Sqlite, SqlitePool};

pub const DB_URL: &str = "sqlite://db.sqlite3";

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

pub async fn create_database() -> Result<SqlitePool, sqlx::Error> {
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

pub async fn create_tables(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
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

pub async fn create_user(pool: &Pool<Sqlite>, name: &str) -> anyhow::Result<()> {
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

pub async fn create_role(
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

pub async fn delete_user(pool: &Pool<Sqlite>, id: i32) -> anyhow::Result<()> {
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

pub async fn delete_role(pool: &Pool<Sqlite>, slug: &str) -> anyhow::Result<()> {
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

pub async fn update_user(pool: &Pool<Sqlite>, id: i32, field: &str, value: &str) -> anyhow::Result<()> {
    let query = format!("UPDATE users SET {} = ? WHERE id = ?", field);

    sqlx::query(&query)
        .bind(value)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_role(
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

pub async fn assign_role(pool: &Pool<Sqlite>, user_id: i32, role_slug: &str) -> anyhow::Result<()> {
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

pub async fn unassign_role(pool: &Pool<Sqlite>, user_id: i32, role_slug: &str) -> anyhow::Result<()> {
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

pub async fn list_roles(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
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

pub async fn list_role(pool: &Pool<Sqlite>, slug: &str) -> anyhow::Result<()> {
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

pub async fn list_users(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
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

pub async fn list_user(pool: &Pool<Sqlite>, id: i32) -> anyhow::Result<()> {
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
