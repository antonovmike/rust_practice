use crate::db::*;

pub async fn handle_command(db: &DataBase, tokens: Vec<&str>) -> anyhow::Result<String> {
    match tokens.as_slice() {
        ["create", "user", name] => {
            let msg = format!("Creating user with name: {}", name);
            println!("{msg}");
            db.create_user(name).await?;
            Ok(msg)
        }
        ["delete", "user", id] => {
            let msg = format!("Deleting user with ID: {}", id);
            println!("{msg}");
            db.delete_user(id.parse()?).await?;
            Ok(msg)
        }
        ["update", "user", id, field, value] => {
            let msg = format!("Updating field {} of user with ID {}: {}", field, id, value);
            println!("{msg}");
            db.update_user(id.parse()?, field, value).await?;
            Ok(msg)
        }
        ["assign", "role", role_slug, "to", "user", user_id] => {
            let msg = format!("Assigning role {} to user with ID {}", role_slug, user_id);
            println!("{msg}");
            db.assign_role(user_id.parse()?, role_slug).await?;
            Ok(msg)
        }
        ["unassign", "role", role_slug, "from", "user", user_id] => {
            let msg = format!(
                "Unassigning role {} from user with ID {}",
                role_slug, user_id
            );
            println!("{msg}");
            db.unassign_role(user_id.parse()?, role_slug).await?;
            Ok(msg)
        }
        ["list", "roles"] => {
            let msg = format!("Listing all roles");
            println!("{msg}");
            db.list_roles().await?;
            Ok(msg)
        }
        ["list", "role", slug] => {
            let msg = format!("Listing role with slug: {}", slug);
            println!("{msg}");
            db.list_role(slug).await?;
            Ok(msg)
        }
        ["list", "users"] => {
            let msg = format!("Listing all users");
            println!("{msg}");
            db.list_users().await?;
            Ok(msg)
        }
        ["list", "user", id] => {
            let msg = format!("Listing user with ID: {}", id);
            println!("{msg}");
            db.list_user(id.parse()?).await?;
            Ok(msg)
        }
        ["create", "role", slug, name, permissions] => {
            let msg = format!(
                "Creating role with slug: {}, name: {}, permissions: {}",
                slug, name, permissions
            );
            println!("{msg}");
            db.create_role(slug, name, permissions).await?;
            Ok(msg)
        }
        ["delete", "role", slug] => {
            let msg = format!("Deleting role with slug: {}", slug);
            println!("{msg}");
            db.delete_role(slug).await?;
            Ok(msg)
        }
        ["update", "role", slug, field, value] => {
            let msg = format!(
                "Updating field {} of role with slug {}: {}",
                field, slug, value
            );
            println!("{msg}");
            db.update_role(slug, field, value).await?;
            Ok(msg)
        }
        _ => {
            println!("Invalid command");
            Err(anyhow::anyhow!("Invalid command"))
        }
    }
}
