use super::*;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use tokio::test;

#[cfg(test)]
async fn setup_db() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite://:memory:")
        .await?;

    create_tables(&pool).await.unwrap();

    Ok(pool)
}

#[cfg(test)]
#[test]
async fn test_create_user() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_user(&pool, "test_user").await?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE name = ?
        "#,
    )
    .bind("test_user")
    .fetch_one(&pool)
    .await?;

    assert_eq!(user.name, "test_user");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_delete_user() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_user(&pool, "test_user").await?;

    delete_user(&pool, 1).await?;

    let result = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&pool)
    .await;

    assert!(result.is_err());

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_create_role() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_role(&pool, "test_role", "Test Role", "read,write").await?;

    let role = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        WHERE slug = ?
        "#,
    )
    .bind("test_role")
    .fetch_one(&pool)
    .await?;

    assert_eq!(role.slug, "test_role");
    assert_eq!(role.name, "Test Role");
    assert_eq!(role.permissions, "read,write");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_delete_role() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_role(&pool, "test_role", "Test Role", "read,write").await?;

    delete_role(&pool, "test_role").await?;

    let result = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        WHERE slug = ?
        "#,
    )
    .bind("test_role")
    .fetch_one(&pool)
    .await;

    assert!(result.is_err());

    Ok(())
}

// #[cfg(test)]
// #[test]
// async fn test_update_role() -> anyhow::Result<()> {
//     let pool = setup_db().await?;

//     create_role(&pool, "test_role", "Test Role", "read,write").await?;

//     update_role(&pool, "test_role", "Updated Role", "read,write,delete").await?;

//     let role = sqlx::query_as::<_, Role>(
//         r#"
//         SELECT slug, name, permissions
//         FROM roles
//         WHERE slug = ?
//         "#,
//     )
//     .bind("test_role")
//     .fetch_one(&pool)
//     .await?;

//     assert_eq!(role.slug, "test_role");
//     assert_eq!(role.name, "Updated Role");
//     assert_eq!(role.permissions, "read,write,delete");

//     Ok(())
// }
