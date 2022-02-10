**include-sqlite-sql** is an extension of [include-sql][1] for using SQLite SQL in Rust. It completes include-sql by providing `impl_sql` macro to generate database access methods from the included SQL. include-sqlite-sql uses [Rusqlite][2] for database access.

# Example

Write your SQL and save it in a file. For example, let's say the following is saved as `library.sql` in the project's `src` folder:

```sql
-- name: get_loaned_books?
-- Returns the list of books loaned to a patron
-- # Parameters
-- param: user_id: &str - user ID
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1;

-- name: loan_books!
-- Updates the book records to reflect loan to a patron
-- # Parameters
-- param: user_id: &str - user ID
-- param: book_ids: u32 - book IDs
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_id IN (:book_ids);
```

And then use it in Rust as:

```rust , ignore
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("src/library.sql");

fn main() -> Result<()> {
    let args : Vec<String> = std::env::args().collect();
    let dbpath = &args[1];
    let user_id = &args[2];

    let db = Connection::open(dbpath)?;

    db.get_loaned_books(user_id, |row| {
        let book_title : &str = row.get_ref("book_title")?.as_str()?;
        println!("{}", book_title);
        Ok(())
    })?;

    Ok(())
}
```

# Documentation

The included [documentation][3] describes the supported SQL file format and provides additional details on the generated code.


[1]: https://crates.io/crates/include-sql
[2]: https://crates.io/crates/rusqlite
[3]: https://quietboil.github.io/include-sqlite-sql
