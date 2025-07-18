**include-sqlite-sql** is an extension of [include-sql][1] for using SQLite SQL in Rust. It completes include-sql by providing `impl_sql` macro to generate database access methods from the included SQL. include-sqlite-sql uses [Rusqlite][2] for database access.

# Usage

Add `include-sqlite-sql` as a dependency:

```toml
[dependencies]
include-sqlite-sql = "0.2"
```

Write your SQL and save it in a file. For example, let's say the following is the content of the `library.sql` file that is saved in the project's `sql` folder:

```sql
-- name: get_loaned_books?
-- Returns the list of books loaned to a patron
-- # Parameters
-- param: user_id: &str - user ID
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1
/
-- name: loan_books!
-- Updates the book records to reflect the loan to a patron
-- # Parameters
-- param: book_titles: &str - book titles
-- param: user_id: &str - user ID
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_title IN (:book_titles)
/
```

And then use it in Rust as:

```rust
use include_sqlite_sql::{include_sql, impl_sql};
use rusqlite::{Result, Connection};

include_sql!("/sql/library.sql");

fn main() -> Result<()> {
    let db = Connection::open("library.db")?;

    db.loan_books(&["War and Peace", "Gone With the Wind"], "Sheldon Cooper")?;

    db.get_loaned_books("Sheldon Cooper", |row| {
        let book_title : &str = row.get_ref("book_title")?.as_str()?;
        println!("{book_title}");
        Ok(())
    })?;

    Ok(())
}
```

> **Note** that the path to the SQL file can be specified relative to the project root, i.e. relative to `CARGO_MANIFEST_DIR`, or relative to the rust module that includes it. 
> * To specify the path to the included SQL file relative to the project root start the path with the `/` character.
> * To specify the path to the included SQL file relative to the rust module that included it start the path with the `./` characters.
> * For compatibility with the legacy code the path to the SQL file can also be specified without `/` or `./` prefix. In this case the path to it will be considered to be relative to the project root (as if it was specified with the leading `/`).

# Anatomy of the Included SQL File

Please see the **Anatomy of the Included SQL File** in [include-sql][4] documentation for the description of the format that include-sql can parse.

# Generated Methods

**include-sqlite-sql** generates 3 variants of database access methods using the following selectors:
* `?` - methods that process rows retrieved by `SELECT`,
* `!` - methods that execute all other non-`SELECT` methods, and
* `&` - methods that execute multiple SQL statements (as a batch), and
* `->` - methods that execute `RETURNING` statements and provide access to returned data.

## Process Selected Rows

For the `SELECT` statement like:

```sql
-- name: get_loaned_books?
-- param: user_id: &str
SELECT book_title FROM library WHERE loaned_to = :user_id;
```

The method with the following signature is generated:

```rust
fn get_loaned_books<F>(&self, user_id: &str, row_callback: F) -> rusqlite::Result<()>
where F: Fn(&rusqlite::Row) -> rusqlite::Result<()>;
```

Where:
- `user_id` is a parameter that has the same name as the SQL parameter with the declared (in the SQL) type as `&str`.
- `F` is a type of a callback (closure) that the method implementation will call to process each row.

## Execute Non-Select Statements

For non-select statements - INSERT, UPDATE, DELETE, etc. - like the following:

```sql
-- name: loan_books!
-- param: user_id: &str
-- param: book_ids: u32
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_id IN (:book_ids);
```

The method with the following signature is generated:

```rust
fn loan_books(&self, user_id: &str, book_ids: &[u32]) -> rusqlite::Result<usize>;
```

Where:
- `user_id` is a parameter that has the same name as the SQL parameter with the declared (in the SQL) type as `&str`,
- `book_ids` is a parameter for the matching IN-list parameter where each item in a collection has type `u32`.

## RETURNING Statements

For DELETE, INSERT, and UPDATE statements that return data via `RETURNING` clause like:

```sql
-- name: add_new_book->
-- param: isbn: &str
-- param: book_title: &str
INSERT INTO library (isbn, book_title)
VALUES (:isbn, :book_title)
RETURNING book_id;
```

The method with the following signature is generated:

```rust
fn add_new_book<F,R>(&self, isbn: &str, book_title: &str, row_callback: F) -> rusqlite::Result<R>
where F: FnOnce(&rusqlite::Row) -> rusqlite::Result<R>;
```

# Inferred Parameter Types

If a statement parameter type is not explicitly specified via `param:`, **include-sqlite-sql** will use `impl rusqlite::ToSql` for the corresponding scalar method parameters. For example, if the SQL from the example above has not provided its parameter type:

```sql
-- name: get_loaned_books?
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1;
```

Then the signature of the generated method would be:

```rust
fn get_loaned_books<F>(&self, user_id: impl rusqlite::ToSql, row_callback: F) -> rusqlite::Result<()>
where F: Fn(&rusqlite::Row) -> rusqlite::Result<()>;
```

For the "IN list" type of parameters **include-sqlite-sql** will generate a method parameter as a slice where each element is the same generic type supplied by **include-sql**:

```sql
-- name: loan_books!
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_id IN (:book_ids);
```

The signature of the generated method would be:

```rust
fn loan_books<BookIds: rusqlite::ToSql>(&self, user_id: impl rusqlite::ToSql, book_ids: &[BookIds]) -> rusqlite::Result<usize>;
```

[1]: https://crates.io/crates/include-sql
[2]: https://crates.io/crates/rusqlite
[3]: https://doc.rust-lang.org/proc_macro/struct.SourceFile.html
[4]: https://quietboil.github.io/include-sql
