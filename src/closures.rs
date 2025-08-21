use rusqlite::Connection;

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

    // conn.execute("DELETE FROM org", ())?;

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

    // conn.execute("DELETE FROM person", ())?;

    println!("test create");

    return Ok(());
}

#[cfg(test)]
mod tests {

    use pretty_sqlite::{print_rows, print_table};
    use rusqlite::{types::Value, Connection, ToSql};
    use serde_json::json;

    use super::*;

    #[test]
    fn it_closures_test() {
        let add  = |x: i32, y: i32 | {
            println!("Returning some text {x} {y}");
            x + y
        };
        let result= add(5, 100);
        println!("{result}");
        
    }

    #[tokio::test]
    async fn it_c06_async_test() {
        const DB_PATH: &str = "_my-db.db3";
        let conn = Connection::open(DB_PATH).unwrap();

        create_scheme(&conn).unwrap();

        let names = &["Jem", "Mike"];

        for name in names {
            for i in 1..10 {
                let name = format!("{name}-{i}");
                let _res = tokio::task::spawn(async move {
                    let conn = Connection::open(DB_PATH).map_err(|err| err.to_string()).unwrap();
                    conn.execute("
                        insert into person (name, yob) values(?1, ?2) 
                    ", (name, &2000))
                    .map_err(|err| err.to_string())
                }).await.unwrap();
            }
        }

        print_table(&conn, "person").unwrap();

        
    }


    #[test]
    fn it_c04_jsonb_test() {
        let conn = Connection::open_in_memory().unwrap();

        create_scheme(&conn).unwrap();

        let data = &[("Jen", 94114), ("Mike", 94115)];
        let mut ids: Vec<i64> = Vec::new();
        for (name, zip ) in data {
            let data_json = json!({
                "address": {
                    "city": "San Francisco",
                    "zip": zip
                }
            });

            let mut stmt = conn.prepare("
                insert into person (name, yob, data_b) 
                    values (?1, ?2, jsonb(?3)) RETURNING id 
            ").unwrap();

            let person_id = stmt.query_row((name, &2000, data_json.to_string()),
                |r| r.get::<_, i64>(0)
                ).unwrap();
            ids.push(person_id);

            let person_1_id = ids.first().ok_or("Should have at least one person").unwrap();
            conn.execute(
                r#"update person set data_b = 
                            jsonb_set(data_b,
                            '$.address.zip', ?2,
                            '$.address.home_owner', json(?3)
                            )
                            where id = ?1
                "#, 
                (&person_1_id, &94222, true.to_string())).unwrap();

            println!("== People owning homes: ");
            let mut stmt = conn.prepare(
                "select id, name, yob, data_b 
                from person 
                where data_b ->> '$.address.home_owner' = :ho",
            ).unwrap();
            let rows = stmt.query(&[(":ho", &true)]).unwrap();
            print_rows(rows).unwrap();

            println!("== People Not owning homes: ");
            let mut stmt = conn.prepare(
                "select name, yob, json(data_b) from person 
                where jsonb_extract(data_b, '$.address.home_owner') is null 
                    or jsonb_extract(data_b, '$.address.home_owner')  = 0 ",
            ).unwrap();
            let rows = stmt.query(()).unwrap();
            print_rows(rows).unwrap();

        }


        print_table(&conn, "person").unwrap();


    }

    #[test]
    fn it_c04_json_test() {
        let conn = Connection::open_in_memory().unwrap();

        create_scheme(&conn).unwrap();

        let data = &[("Jen", 94114), ("Mike", 94115)];
        let mut ids: Vec<i64> = Vec::new();
        for (name, zip ) in data {
            let data_json = json!({
                "address": {
                    "city": "San Francisco",
                    "zip": zip
                }
            });

            let mut stmt = conn.prepare("
                insert into person (name, yob, data_t) 
                    values (?1, ?2, ?3) RETURNING id 
            ").unwrap();

            let person_id = stmt.query_row((name, &2000, data_json.to_string()),
                |r| r.get::<_, i64>(0)
                ).unwrap();
            ids.push(person_id);

            let person_1_id = ids.first().ok_or("Should have at least one person").unwrap();
            conn.execute(
                r#"update person set data_t = 
                            json_set(data_t,
                            '$.address.zip', ?2,
                            '$.address.home_owner', json(?3)
                            )
                            where id = ?1
                "#, 
                (&person_1_id, &94222, true.to_string())).unwrap();

            println!("== People owning homes: ");
            let mut stmt = conn.prepare(
                "select id, name, yob, data_t 
                from person 
                where json_extract(data_t, '$.address.home_owner') = :ho",
            ).unwrap();
            let rows = stmt.query(&[(":ho", &true)]).unwrap();
            print_rows(rows).unwrap();

            println!("== People Not owning homes: ");
            let mut stmt = conn.prepare(
                "select * from person 
                where json_extract(data_t, '$.address.home_owner') is null 
                    or json_extract(data_t, '$.address.home_owner') = 0 ",
            ).unwrap();
            let rows = stmt.query(()).unwrap();
            print_rows(rows).unwrap();

        }


        print_table(&conn, "person").unwrap();


    }

    #[test]
    fn it_values_test() {
        let conn = Connection::open_in_memory().unwrap();

        create_scheme(&conn).unwrap();

        let names = &["Jem", "Mike", "Paul", "Pierre"];
        for (_idx, name) in names.iter().enumerate() {
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
        print_table(&conn, "org").unwrap();
        print_table(&conn, "person").unwrap();

        let mut stmt = conn.prepare("
            insert into org(name) values (?1) RETURNING id
        ").unwrap();
        let org_id = stmt.query_row(&[("ACME, Inc")],
        |r| r.get::<_, i64>(0)).unwrap();

        print_table(&conn, "org").unwrap();
        println!("org id: {0}", org_id);
        
        let names = &["Jem", "Mike", "Paul", "Pierre"];
        for (idx, name) in names.iter().enumerate() {
            let org_id = if idx % 2 == 0 { Some(org_id) } else { None };
            conn.execute("insert into person (name, org_id, yob) values (?1, ?2,?3)", 
                (name, &org_id, &2000)).unwrap();
        }
        print_table(&conn, "person").unwrap();

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
        ", ()).unwrap();

        conn.execute("insert into person (name, yob) values (?1, ?2)",
             ("Jen", &2000)).unwrap();

        pretty_sqlite::print_table(&conn, "person").unwrap();
        
        let select_sql = "select id, name, yob 
            from person 
            where yob > :yob";

        let mut stmt = conn.prepare(select_sql).unwrap();
        let mut rows = stmt.query(&[(":yob", &1900)]).unwrap();

        pretty_sqlite::print_select(&conn, select_sql, &[(":yob", &1900)]).unwrap();

        
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