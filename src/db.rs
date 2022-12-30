use std::path::Path;
use std::rc::Rc;

use rusqlite::Connection;

use crate::errors;

pub type DB = Rc<Connection>;

pub fn get_connection() -> Result<DB, errors::Error> {
    let path = Path::new("compliments.db");

    Ok(Rc::new(Connection::open(path)?))
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

pub fn add_compliment(conn: DB, compliment: String) -> Result<(), errors::Error> {
    conn.execute(
        "INSERT INTO compliments (compliment) VALUES (?)",
        [compliment],
    )?;

    Ok(())
}

pub fn add_compliments(conn: DB, compliments: Vec<String>) -> Vec<(String, errors::Error)> {
    let mut errors = Vec::new();

    for compliment in compliments {
        match add_compliment(conn.clone(), compliment.clone()) {
            Ok(_) => {}
            Err(error) => errors.push((compliment, error)),
        }
    }

    errors
}

pub fn get_random_compliment(conn: DB) -> Result<String, errors::Error> {
    let compliment = conn.query_row(
        "SELECT compliment FROM compliments ORDER BY random() LIMIT 1",
        [],
        |row| row.get(0),
    )?;

    Ok(compliment)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rusqlite::Connection;

    use super::*;

    fn setup_test_db() -> DB {
        let db = get_in_memory_connection().unwrap();

        initialize_db(db.clone()).unwrap();

        db
    }

    #[test]
    fn test_initialize_db() {
        let db = get_in_memory_connection().unwrap();

        initialize_db(db.clone()).unwrap();
    }

    #[test]
    fn test_add_compliment() {
        let db_conn = setup_test_db();

        let compliment = "Your fit looks nice".to_string();

        let result = add_compliment(db_conn.clone(), compliment).unwrap();

        assert_eq!(result, ());
    }

    #[test]
    fn test_add_compliments() {
        let db_conn = setup_test_db();

        let compliment = "Your fit looks nice".to_string();
        let compliments = vec![compliment.clone(), compliment.clone()];

        let result = add_compliments(db_conn.clone(), compliments);

        dbg!(&result);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_get_random_compliment() {
        let db_conn = setup_test_db();

        let compliment = "Random is not random if there is only one row in the db".to_string();

        let _ = add_compliment(db_conn.clone(), compliment.clone()).unwrap();
        let result = get_random_compliment(db_conn.clone()).unwrap();

        assert_eq!(result, compliment);
    }
}
