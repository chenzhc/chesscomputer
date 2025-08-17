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

    use pretty_sqlite::print_rows;
    use rusqlite::{types::Value, Connection, ToSql};

    use super::*;

    #[test]
    fn it_values_test() {
        let conn = Connection::open_in_memory().unwrap();

        create_scheme(&conn).unwrap();

        let names = &["Jem", "Mike", "Paul", "Pierre"];
        for (idx, name) in names.iter().enumerate() {
            let org_id: Option<i64> = None ;
            conn.execute("insert into person (name, org_id, yob) values (?1, ?2,?3)", 
                (name, &org_id, &2000)).unwrap();
        }

        let nv_list = vec![
            ("org_id".to_string(), Value::Integer(123)),
            ("name".to_string(), Value::Text("New name 111".to_string())),
        ];

        let (cols, vals): (Vec<String>, Vec<Value>) = nv_list.into_iter().unzip();
        let cols = cols.iter().map(|col| format!("\"{}\" = ?", col))
            .collect::<Vec<_>>().join(", ");

        let sql = format!("update person set {cols}");
        let mut values: Vec<&dyn ToSql>  = vals.iter().map(|x| x as &dyn ToSql).collect();

        let sql = format!("{sql}  where id = ?");
        let person_id = Value::Integer(1);
        values.push(&person_id);

        let num_of_rows = conn.execute(&sql, &*values).unwrap();
        println!("number on rows updated: {num_of_rows}");

        print_table(&conn, "person").unwrap();

    }

    #[test]
    fn it_c02_join_test() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = create_scheme(&conn);
        print_table(&conn, "org");
        print_table(&conn, "person");

        let mut stmt = conn.prepare("
            insert into org(name) values (?1) RETURNING id
        ").unwrap();
        let org_id = stmt.query_row(&[("ACME, Inc")],
        |r| r.get::<_, i64>(0)).unwrap();

        print_table(&conn, "org");
        println!("org id: {0}", org_id);
        
        let names = &["Jem", "Mike", "Paul", "Pierre"];
        for (idx, name) in names.iter().enumerate() {
            let org_id = if idx % 2 == 0 { Some(org_id) } else { None };
            conn.execute("insert into person (name, org_id, yob) values (?1, ?2,?3)", 
                (name, &org_id, &2000)).unwrap();
        }
        print_table(&conn, "person");

        let query = "
            select t1.id , t1.name, t1.yob,
                t2.name as org_name 
            from person t1 
            inner join org t2 on t1.org_id = t2.id 
            where t2.id = :org_id 
        ";

        let mut stmt = conn.prepare(query).unwrap();
        let rows = stmt.query(&[(":org_id", &org_id)]).unwrap();
        print_rows(rows).unwrap();


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