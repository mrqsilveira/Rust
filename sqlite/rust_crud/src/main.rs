use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connect to SQLite database (or create it if it doesn’t exist)
    let conn = Connection::open("contacts.db")?;

    // Create the "contacts" table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            phone TEXT NOT NULL,
            email TEXT
        )",
        [],
    )?;

    // Insert a new contact
    add_contact(&conn, "Alice", "123-456-7890", "alice@example.com")?;

    // Retrieve and display all contacts
    let contacts = get_contacts(&conn)?;
    for contact in contacts {
        println!("{:?}", contact);
    }

    // Update a contact
    update_contact(&conn, 1, "Alice Updated", "987-654-3210", "updated@example.com")?;

    // Delete a contact
    delete_contact(&conn, 1)?;

    Ok(())
}

// Add a new contact
fn add_contact(conn: &Connection, name: &str, phone: &str, email: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO contacts (name, phone, email) VALUES (?1, ?2, ?3)",
        params![name, phone, email],
    )?;
    Ok(())
}

// Get all contacts
fn get_contacts(conn: &Connection) -> Result<Vec<(i32, String, String, String)>> {
    let mut stmt = conn.prepare("SELECT id, name, phone, email FROM contacts")?;
    let contacts = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(contacts)
}

// Update a contact
fn update_contact(conn: &Connection, id: i32, name: &str, phone: &str, email: &str) -> Result<()> {
    conn.execute(
        "UPDATE contacts SET name = ?1, phone = ?2, email = ?3 WHERE id = ?4",
        params![name, phone, email, id],
    )?;
    Ok(())
}

// Delete a contact
fn delete_contact(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM contacts WHERE id = ?1", params![id])?;
    Ok(())
}
