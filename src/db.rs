use std::path::Path;
use std::rc::Rc;

use rusqlite::Connection;

use crate::errors;

pub type DB = Rc<Connection>;

pub fn get_connection() -> Result<Connection, errors::Error> {
    let path = Path::new("compliments.db");

    Ok(Connection::open(path)?)
}

pub fn get_in_memory_connection() -> Result<DB, errors::Error> {
    let conn = Connection::open_in_memory()?;

    Ok(Rc::new(conn))
}

pub fn initialize_db(conn: DB) -> Result<(), errors::Error> {
    conn.execute_batch(
        "
            BEGIN TRANSACTION;
            DROP TABLE IF EXISTS compliments;
            CREATE TABLE IF NOT EXISTS compliments (
	            id	INTEGER NOT NULL UNIQUE,
	            compliment	TEXT NOT NULL UNIQUE,
	            PRIMARY KEY(id)
            );
            COMMIT;
        ",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rusqlite::Connection;

    use super::*;

    #[test]
    fn test_initialize_db() {
        let db = get_in_memory_connection().unwrap();

        initialize_db(db.clone()).unwrap();
    }
}
