use super::*;
use tokio::test;

#[cfg(test)]
async fn setup_db() -> Result<DataBase, sqlx::Error> {
    let db = DataBase::create_database("sqlite://:memory:").await?;
    db.create_tables().await.unwrap();
    Ok(db)
}

#[cfg(test)]
#[test]
async fn test_create_user() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user").await?;

    let user = sqlx::query_as::<_, User>(
        r#"
       SELECT id, name
       FROM users
       WHERE name = ?
       "#,
    )
    .bind("test_user")
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(user.name, "test_user");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_delete_user() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user").await?;
    db.delete_user(1).await?;

    let result = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&db.pool)
    .await;

    assert!(result.is_err());

    Ok(())
}

#[cfg(test)]
#[tokio::test]
async fn test_update_user() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user").await?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE name = ?
        "#,
    )
    .bind("test_user")
    .fetch_one(&db.pool)
    .await?;

    db.update_user(user.id, "name", "updated_user").await?;

    let updated_user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user.id)
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(updated_user.name, "updated_user");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_create_role() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_role("test_role", "Test Role", "read,write")
        .await?;

    let role = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        WHERE slug = ?
        "#,
    )
    .bind("test_role")
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(role.slug, "test_role");
    assert_eq!(role.name, "Test Role");
    assert_eq!(role.permissions, "read,write");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_delete_role() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_role("test_role", "Test Role", "read,write")
        .await?;
    db.delete_role("test_role").await?;

    let result = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        WHERE slug = ?
        "#,
    )
    .bind("test_role")
    .fetch_one(&db.pool)
    .await;

    assert!(result.is_err());

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_update_role() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_role("test_role", "Test Role", "read,write")
        .await?;
    db.update_role("test_role", "permissions", "read,write,delete")
        .await?;

    let role = sqlx::query_as::<_, Role>(
        r#"
       SELECT slug, name, permissions
       FROM roles
       WHERE slug = ?
       "#,
    )
    .bind("test_role")
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(role.slug, "test_role");
    assert_eq!(role.name, "Test Role");
    assert_eq!(role.permissions, "read,write,delete");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_assign_role() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user").await?;
    db.create_role("test_role", "Test Role", "read,write")
        .await?;
    db.assign_role(1, "test_role").await?;

    let user_role = sqlx::query(
        r#"
        SELECT roles.slug
        FROM roles
        JOIN users_roles ON roles.slug = users_roles.role_slug
        WHERE users_roles.user_id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&db.pool)
    .await?;

    let slug: String = sqlx::Row::get(&user_role, "slug");
    assert_eq!(slug, "test_role");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_unassign_role() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user").await?;
    db.create_role("test_role", "Test Role", "read,write")
        .await?;
    db.assign_role(1, "test_role").await?;
    db.unassign_role(1, "test_role").await?;

    let result = sqlx::query(
        r#"
        SELECT roles.slug
        FROM roles
        JOIN users_roles ON roles.slug = users_roles.role_slug
        WHERE users_roles.user_id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&db.pool)
    .await;

    assert!(result.is_err());

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_list_roles() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_role("test_role1", "Test Role 1", "read,write")
        .await?;
    db.create_role("test_role2", "Test Role 2", "read,write")
        .await?;
    db.list_roles().await?;

    let roles = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        "#,
    )
    .fetch_all(&db.pool)
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
    let db = setup_db().await?;

    db.create_role("test_role", "Test Role", "read,write")
        .await?;
    db.list_role("test_role").await?;

    let role = sqlx::query_as::<_, Role>(
        r#"
        SELECT slug, name, permissions
        FROM roles
        WHERE slug = ?
        "#,
    )
    .bind("test_role")
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(role.slug, "test_role");
    assert_eq!(role.name, "Test Role");
    assert_eq!(role.permissions, "read,write");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_list_users() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user1").await?;
    db.create_user("test_user2").await?;
    db.list_users().await?;

    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        "#,
    )
    .fetch_all(&db.pool)
    .await?;

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "test_user1");
    assert_eq!(users[1].name, "test_user2");

    Ok(())
}

#[cfg(test)]
#[test]
async fn test_list_user() -> anyhow::Result<()> {
    let db = setup_db().await?;

    db.create_user("test_user").await?;
    db.list_user(1).await?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(1)
    .fetch_one(&db.pool)
    .await?;

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "test_user");

    Ok(())
}
