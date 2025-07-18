use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("/tests/init.sql");
include_sql!("/tests/returning.sql");

#[test]
fn updates() -> Result<()> {
    let db = Connection::open_in_memory()?;

    db.create_test_table()?;
    db.insert_test_quotes()?;

    let id = db.insert_new_quote("Benjamin Franklin", "Tell me and I forget. Teach me and I remember. Involve me and I learn.", |row| {
        let id : i32 = row.get(0)?;
        Ok(id)
    })?;

    let mut row_num = 0;
    db.get_quote_by_id(id, |row| {
        let author: &str = row.get_ref(0)?.as_str()?;
        let quote : &str = row.get_ref(1)?.as_str()?;
        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(author, "Benjamin Franklin");
                assert_eq!(quote, "Tell me and I forget. Teach me and I remember. Involve me and I learn.");
            },
            _ => {
                panic!("one row was expected");
            }
        }
        Ok(())
    })?;
    assert_eq!(row_num, 1);

    let id = db.insert_new_quote_gen("Aristotle", "It is during our darkest moments that we must focus to see the light.", |row| {
        let id : i32 = row.get(0)?;
        Ok(id)
    })?;

    let mut row_num = 0;
    db.get_quote_by_id(id, |row| {
        let author: &str = row.get_ref(0)?.as_str()?;
        let quote : &str = row.get_ref(1)?.as_str()?;
        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(author, "Aristotle");
                assert_eq!(quote, "It is during our darkest moments that we must focus to see the light.");
            },
            _ => {
                panic!("one row was expected");
            }
        }
        Ok(())
    })?;
    assert_eq!(row_num, 1);

    Ok(())
}