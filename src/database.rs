use anyhow::Result;
use log::{error, info};
use rusqlite::{Connection, params};

const DB_NAME: &str = "ownership_changes.db";

/// Initialize the SQLite database
pub fn init_database() -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ownership_changes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            original_owner TEXT NOT NULL,
            current_owner TEXT NOT NULL,
            operation_time DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    info!("Database initialized");
    Ok(())
}

/// Record a change in ownership
pub fn record_change(path: &str, original_owner: &str, current_owner: &str) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    
    conn.execute(
        "INSERT INTO ownership_changes (path, original_owner, current_owner) VALUES (?1, ?2, ?3)",
        params![path, original_owner, current_owner],
    )?;

    info!("Recorded change: {} -> {} for {}", original_owner, current_owner, path);
    Ok(())
}

/// Get all recorded changes
pub fn get_all_changes() -> Result<Vec<(i32, String, String, String, String)>> {
    let conn = Connection::open(DB_NAME)?;
    let mut stmt = conn.prepare(
        "SELECT id, path, original_owner, current_owner, operation_time FROM ownership_changes",
    )?;

    let records = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

/// Delete a specific change record
pub fn delete_change(id: i32) -> Result<()> {
    let conn = Connection::open(DB_NAME)?;
    
    conn.execute(
        "DELETE FROM ownership_changes WHERE id = ?1",
        params![id],
    )?;

    info!("Deleted change record: {}", id);
    Ok(())
}
