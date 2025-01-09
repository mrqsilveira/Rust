use rusqlite::{params, Connection, Result};

/// Initialize the database and create the contacts table if it doesn't exist
pub fn init_database(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            phone TEXT NOT NULL,
            email TEXT
        )",
        [],
    )?;
    Ok(conn)
}

/// Add a new contact
pub fn add_contact(conn: &Connection, name: &str, phone: &str, email: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO contacts (name, phone, email) VALUES (?1, ?2, ?3)",
        params![name, phone, email],
    )?;
    Ok(())
}

/// Get all contacts
pub fn get_contacts(conn: &Connection) -> Result<Vec<(i32, String, String, String)>> {
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

/// Update a contact
pub fn update_contact(conn: &Connection, id: i32, name: &str, phone: &str, email: &str) -> Result<()> {
    conn.execute(
        "UPDATE contacts SET name = ?1, phone = ?2, email = ?3 WHERE id = ?4",
        params![name, phone, email, id],
    )?;
    Ok(())
}

/// Delete a contact
pub fn delete_contact(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM contacts WHERE id = ?1", params![id])?;
    Ok(())
}
