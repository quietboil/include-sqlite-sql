use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("/examples/library.sql");

fn main() -> Result<()> {
    let db = Connection::open_in_memory()?;

    db.init_library()?;

    db.loan_books(&["War and Peace", "Gone With the Wind"], "Sheldon Cooper")?;
    db.loan_books(&["The Lord of the Rings", "Master and Commander"], "Leonard Hofstadter")?;

    db.get_loaned_books("Sheldon Cooper", |row| {
        let book_title : &str = row.get_ref(0)?.as_str()?;
        println!("{book_title}");
        Ok(())
    })?;

    println!("---");

    db.get_loaned_books("Leonard Hofstadter", |row| {
        let book_title : &str = row.get_ref(0)?.as_str()?;
        println!("{book_title}");
        Ok(())
    })?;

    Ok(())
}
