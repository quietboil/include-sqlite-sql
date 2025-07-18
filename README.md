[![crates.io](https://img.shields.io/crates/v/include-sqlite-sql)](https://crates.io/crates/include-sqlite-sql)
[![Documentation](https://docs.rs/include-sqlite-sql/badge.svg)](https://docs.rs/include-sqlite-sql)
![MIT](https://img.shields.io/crates/l/include-sqlite-sql.svg)

**include-sqlite-sql** is an extension of [include-sql][1] for using SQLite SQL in Rust. It completes include-sql by providing `impl_sql` macro to generate database access methods from the included SQL. include-sqlite-sql uses [Rusqlite][2] for database access.

# Example

Write your SQL and save it in a file. For example, let's say the following is saved as `library.sql` in the project's `sql` folder:

```sql
-- name: get_loaned_books ?
--
-- Returns the list of books loaned to a patron
--
-- # Parameters
--
-- param: user_id: &str - user ID
--
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1

-- name: loan_books!
--
-- Updates the book records to reflect loan to a patron
--
-- # Parameters
--
-- param: book_titles: &str - book titles
-- param: user_id: &str - user ID
--
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_title IN (:book_titles)
```

And then use it in Rust as:

```rust , ignore
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("/sql/library.sql");

fn main() -> Result<()> {
    let db = Connection::open("library.db")?;

    db.loan_books(&["War and Peace", "Gone With the Wind"], "Sheldon Cooper")?;

    db.get_loaned_books("Sheldon Cooper", |row| {
        let book_title : &str = row.get_ref(0)?.as_str()?;
        println!("{book_title}");
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
[4]: https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html#method.execute_batch
[5]: examples/library.sql