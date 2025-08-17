use rusqlite::Connection;
use pretty_sqlite::print_table;

pub fn test_closeures() {
    let add = || println!("Returing some text");

    add(); 

}

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;


pub fn create_scheme(conn: &Connection) -> Result<()> {

    conn.execute(
        "CREATE TABLE IF NOT EXISTS org (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            ) STRICT
        ", ())?;

    conn.execute("DELETE FROM org", ())?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            org_id INTEGER,
            name TEXT NOT NULL,
            yob INTEGER,
            data_t TEXT,
            data_b BLOB
        ) STRICT
    ", ())?;

    conn.execute("DELETE FROM person", ())?;

    println!("test create");

    return Ok(());
}

#[cfg(test)]
mod tests {

    use rusqlite::Connection;

    use super::*;

    #[test]
    fn it_c02_join_test() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = create_scheme(&conn);
        print_table(&conn, "org");
        print_table(&conn, "person");


    }

    #[test]
    fn it_conn_db_test() {
        // -- Memory SQLite.
        // let conn = Connection::open("my-db.db3").unwrap();
        let conn = Connection::open_in_memory().unwrap();

        conn.execute("
            CREATE TABLE person(
                id INTEGER PRIMARY KEY autoincrement,
                name TEXT NOT NULL,
                yob INTEGER, -- year of birth
                data BLOB
            ) STRICT
        ", ());

        conn.execute("insert into person (name, yob) values (?1, ?2)",
             ("Jen", &2000));

        pretty_sqlite::print_table(&conn, "person");
        
        let select_sql = "select id, name, yob 
            from person 
            where yob > :yob";

        let mut stmt = conn.prepare(select_sql).unwrap();
        let mut rows = stmt.query(&[(":yob", &1900)]).unwrap();

        pretty_sqlite::print_select(&conn, select_sql, &[(":yob", &1900)]);

        
        while let Some(row) = rows.next().unwrap() {
            let name: String = row.get(1).unwrap();
            println!("name: {name}"); 
            println!("rows: {:?}", row);
        }
    }

    #[test]
    fn it_test02() {
        println!("test");
        test_closeures();

    }

}