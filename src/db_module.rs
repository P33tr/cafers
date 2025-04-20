use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Cafe {
    pub name: String,
    pub description: String,
}

// Function to execute the SQL SELECT query
pub fn get_cafes(conn: &Connection) -> Result<Vec<Cafe>> {
    let mut stmt = conn.prepare("SELECT name, description FROM cafe")?;
    let cafe_iter = stmt.query_map([], |row| {
        Ok(Cafe {
            name: row.get(0)?,
            description: row.get(1)?,
        })
    })?;

    let mut cafes = Vec::new();
    for cafe in cafe_iter {
        cafes.push(cafe?);
    }
    Ok(cafes)
}