mod db_module;

use rusqlite::{Connection, Result};
use db_module::{get_cafes, Cafe};


fn main() -> Result<()> {


    #[derive(Debug)]
    struct Cafe {
        name: String,
        description: String
    }
    let conn = Connection::open("my_database.db")?;

    // fetch cafes using the module
    let cafes = get_cafes(&conn)?;


    for cafe in cafes {
        println!("Found cafe {:?}", cafe);
    }

    Ok(())
}
