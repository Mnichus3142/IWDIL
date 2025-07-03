use rusqlite::Result;

pub fn init() -> Result<()> {
    let conn = rusqlite::Connection::open("iwdil.sqlite")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        description TEXT NOT NULL,
        completed BOOLEAN NOT NULL DEFAULT 0
    )", [])?;

    Ok(())
}