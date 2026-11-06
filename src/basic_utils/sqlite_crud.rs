use rusqlite::{Connection, Result};

#[derive(Debug)]
struct UserProfile {
    id: i32,
    name: String,
    email: String,
}

/// Demonstrates create, read, update, and delete operations in an in-memory SQLite database.
pub fn run() -> Result<()> {
    // 1. Connect to an in-memory SQLite database
    let conn = Connection::open_in_memory()?;
    //let conn = Connection::open("info.db")?;

    // 2. CREATE: Create the users table
    conn.execute(
        "CREATE TABLE users (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  name TEXT NOT NULL,
                  email TEXT NOT NULL
                  )",
        [],
    )?;

    // Insert a record for Uday Shiwakoti
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        ("Uday Shiwakoti", "uday@uday.com.np"),
    )?;
    println!("-> [CREATE] Inserted user record successfully for Uday Shiwakoti.");

    // 3. READ: Query and display all user records
    let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(UserProfile {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
        })
    })?;

    println!("-> [READ] Fetching users from in-memory DB:");
    for user in user_iter {
        let user: UserProfile = user?;

        println!("   Found user: {:?} ", user);
        println!(
            "ID: {}, Name: {}, Email: {}",
            user.id, user.name, user.email
        );
    }

    // 4. UPDATE: Modify the user's email address
    conn.execute(
        "UPDATE users SET email = ?1 WHERE name = ?2",
        ("updated_uday@uday.com.np", "Uday Shiwakoti"),
    )?;
    println!("-> [UPDATE] Updated user email successfully.");

    // Verify the update
    let updated_email: String = conn.query_row(
        "SELECT email FROM users WHERE name = ?1",
        ["Uday Shiwakoti"],
        |row| row.get(0),
    )?;
    println!("   Verified updated email: {}", updated_email);

    // 5. DELETE: Remove the user record
    conn.execute("DELETE FROM users WHERE name = ?1", ["Uday Shiwakoti"])?;
    println!("-> [DELETE] Deleted user record successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn completes_crud_operations_in_memory() {
        assert!(run().is_ok());
    }
}
