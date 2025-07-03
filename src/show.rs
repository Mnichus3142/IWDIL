use rusqlite::Result;

pub fn show () -> Result<()> {
    let conn = rusqlite::Connection::open("iwdil.sqlite")?;
    Ok(())
}