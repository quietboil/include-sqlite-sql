use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("tests/init.sql");
include_sql!("tests/updates.sql");

#[test]
fn updates() -> Result<()> {
    let db = Connection::open(":memory:")?;

    db.create_test_table()?;
    db.insert_test_quotes()?;

    let count = db.insert_new_quote("John Lennon", "Life is what happens when you're busy making other plans.")?;
    assert_eq!(count, 1);

    let count = db.insert_new_quote_gen("Franklin D. Roosevelt", "When you reach the end of your rope, tie a knot in it and hang on.")?;
    assert_eq!(count, 1);

    let mut row_num = 0;
    db.get_quotes_by_author("John Lennon", |row| {
        let quote: &str = row.get_ref(0)?.as_str()?;
        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(quote, "Life is what happens when you're busy making other plans.");
            },
            _ => {
                panic!("one row was expected");
            }
        }
        Ok(())
    })?;
    assert_eq!(row_num, 1);

    let mut row_num = 0;
    db.get_quotes_by_author("Franklin D. Roosevelt", |row| {
        let quote: &str = row.get_ref(0)?.as_str()?;
        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(quote, "When you reach the end of your rope, tie a knot in it and hang on.");
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