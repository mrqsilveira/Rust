use rusqlite::Result;

// Import the renamed module
mod mod_sqlite;

fn main() -> Result<()> {
    // Create a new database connection
    let conn = mod_sqlite::init_database("contacts.db")?;

    // Add a new contact
    mod_sqlite::add_contact(&conn, "Alice", "123-456-7890", "alice@example.com")?;

    // Retrieve and display all contacts
    let contacts = mod_sqlite::get_contacts(&conn)?;
    for contact in contacts {
        println!("{:?}", contact);
    }

    // Update a contact
    mod_sqlite::update_contact(&conn, 1, "Alice Updated", "987-654-3210", "updated@example.com")?;

    // Delete a contact
    mod_sqlite::delete_contact(&conn, 1)?;

    Ok(())
}
