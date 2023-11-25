use crate::db::*;

pub async fn handle_command(db: &DataBase, tokens: Vec<&str>) -> anyhow::Result<()> {
    match tokens.as_slice() {
        ["create", "user", name] => {
            println!("Creating user with name: {}", name);
            db.create_user(name).await?;
            Ok(())
        }
        ["delete", "user", id] => {
            println!("Deleting user with ID: {}", id);
            db.delete_user(id.parse()?).await?;
            Ok(())
        }
        ["update", "user", id, field, value] => {
            println!("Updating field {} of user with ID {}: {}", field, id, value);
            db.update_user(id.parse()?, field, value).await?;
            Ok(())
        }
        ["assign", "role", role_slug, "to", "user", user_id] => {
            println!("Assigning role {} to user with ID {}", role_slug, user_id);
            db.assign_role(user_id.parse()?, role_slug).await?;
            Ok(())
        }
        ["unassign", "role", role_slug, "from", "user", user_id] => {
            println!(
                "Unassigning role {} from user with ID {}",
                role_slug, user_id
            );
            db.unassign_role(user_id.parse()?, role_slug).await?;
            Ok(())
        }
        ["list", "roles"] => {
            println!("Listing all roles");
            db.list_roles().await?;
            Ok(())
        }
        ["list", "role", slug] => {
            println!("Listing role with slug: {}", slug);
            db.list_role(slug).await?;
            Ok(())
        }
        ["list", "users"] => {
            println!("Listing all users");
            db.list_users().await?;
            Ok(())
        }
        ["list", "user", id] => {
            println!("Listing user with ID: {}", id);
            db.list_user(id.parse()?).await?;
            Ok(())
        }
        ["create", "role", slug, name, permissions] => {
            println!(
                "Creating role with slug: {}, name: {}, permissions: {}",
                slug, name, permissions
            );
            db.create_role(slug, name, permissions).await?;
            Ok(())
        }
        ["delete", "role", slug] => {
            println!("Deleting role with slug: {}", slug);
            db.delete_role(slug).await?;
            Ok(())
        }
        ["update", "role", slug, field, value] => {
            println!(
                "Updating field {} of role with slug {}: {}",
                field, slug, value
            );
            db.update_role(slug, field, value).await?;
            Ok(())
        }
        _ => {
            println!("Invalid command");
            Err(anyhow::anyhow!("Invalid command"))
        }
    }
}
