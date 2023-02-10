use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("tests/init.sql");
include_sql!("tests/queries.sql");

#[test]
fn queries() -> Result<()> {
    let db = Connection::open(":memory:")?;

    db.create_test_table()?;
    db.insert_test_quotes()?;

    db.count_quotes(|row| {
        let count : i32 = row.get(0)?;
        assert_eq!(count, 10);
        Ok(())
    })?;

    let mut row_num = 0;
    db.get_quotes_longer_than(72, |row| {
        let author : &str = row.get_ref(0)?.as_str()?;
        let quote  : &str = row.get_ref(1)?.as_str()?;
        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(author, "Albert Einstein");
                assert_eq!(quote, "Life is like riding a bicycle. To keep your balance, you must keep moving.");
            },
            _ => {
                panic!("one row was expected");
            }
        }
        Ok(())
    })?;
    assert_eq!(row_num, 1);

    let mut row_num = 0;
    db.get_quotes_within_range(60, 70, |row| {
        let author : &str = row.get_ref(0)?.as_str()?;
        let quote  : &str = row.get_ref(1)?.as_str()?;
        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(author, "George Orwell");
                assert_eq!(quote, "Freedom is the right to tell people what they do not want to hear.");
            },
            2 => {
                assert_eq!(author, "Vincent Van Gogh");
                assert_eq!(quote, "Great things are done by a series of small things brought together.");
            },
            _ => {
                panic!("two rows were expected");
            }
        }
        Ok(())
    })?;
    assert_eq!(row_num, 2);

    Ok(())
}