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

#[cfg(test)]
#[test]
async fn test_update_role() -> anyhow::Result<()> {
   let pool = setup_db().await?;

   create_role(&pool, "test_role", "Test Role", "read,write").await?;
   update_role(&pool, "test_role", "permissions", "read,write,delete").await?;

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
   assert_eq!(role.permissions, "read,write,delete");

   Ok(())
}

#[cfg(test)]
#[test]
async fn test_assign_role() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_user(&pool, "test_user").await?;
    create_role(&pool, "test_role", "Test Role", "read,write").await?;

    assign_role(&pool, 1, "test_role").await?;

    let user_role = sqlx::query(
        r#"
        SELECT roles.slug
        FROM roles
        JOIN users_roles ON roles.slug = users_roles.role_slug
        WHERE users_roles.user_id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&pool)
    .await?;

    let slug: String = sqlx::Row::get(&user_role, "slug");
    assert_eq!(slug, "test_role");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_unassign_role() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_user(&pool, "test_user").await?;
    create_role(&pool, "test_role", "Test Role", "read,write").await?;
    assign_role(&pool, 1, "test_role").await?;

    unassign_role(&pool, 1, "test_role").await?;

    let result = sqlx::query(
        r#"
        SELECT roles.slug
        FROM roles
        JOIN users_roles ON roles.slug = users_roles.role_slug
        WHERE users_roles.user_id = ?
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
async fn test_list_roles() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_role(&pool, "test_role1", "Test Role 1", "read,write").await?;
    create_role(&pool, "test_role2", "Test Role 2", "read,write").await?;

    list_roles(&pool).await?;

    let roles = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        "#,
    )
    .fetch_all(&pool)
    .await?;

    assert_eq!(roles.len(), 2);
    assert_eq!(roles[0].slug, "test_role1");
    assert_eq!(roles[0].name, "Test Role 1");
    assert_eq!(roles[0].permissions, "read,write");
    assert_eq!(roles[1].slug, "test_role2");
    assert_eq!(roles[1].name, "Test Role 2");
    assert_eq!(roles[1].permissions, "read,write");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_list_role() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_role(&pool, "test_role", "Test Role", "read,write").await?;

    list_role(&pool, "test_role").await?;

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
async fn test_list_users() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_user(&pool, "test_user1").await?;
    create_user(&pool, "test_user2").await?;

    list_users(&pool).await?;

    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        "#,
    )
    .fetch_all(&pool)
    .await?;

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "test_user1");
    assert_eq!(users[1].name, "test_user2");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_list_user() -> anyhow::Result<()> {
    let pool = setup_db().await?;

    create_user(&pool, "test_user").await?;

    list_user(&pool, 1).await?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&pool)
    .await?;

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "test_user");

    Ok(())
}
