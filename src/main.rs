use rusqlite::{Connection, Result};

fn main() -> Result<()> {


    #[derive(Debug)]
    struct Cafe {
        name: String,
        description: String
    }
    let conn = Connection::open("my_database.db")?;

    // Additional operations such as creating tables or inserting data can be done here
   // let mut stmt = conn.prepare("INSERT INTO cafe (name, description) VALUES (?,?)")?;
   // stmt.execute(["petersCafe", "the nice place"])?;


    let mut stmt = conn.prepare("SELECT  name, description FROM cafe")?;
    let cafe_iter = stmt.query_map([], |row| {
        Ok(Cafe {
            name: row.get(0)?,
            description: row.get(1)?,
        })
    })?;

    for cafe in cafe_iter {
        println!("Found cafe {:?}", cafe?);
    }

    Ok(())
}
