#![cfg_attr(docsrs, doc = include_str!("../docs/index.md"))]

pub use include_sql::include_sql;

/**
Generates Rust code to use included SQL.

This macro defines a trait with methods to access data and implements it for `rusqlite::Connection`.

This macro recognizes and generates 3 variants of database access methods using the following selectors:
* `?` - methods that process rows retrieved by `SELECT`,
* `!` - methods that execute all other non-`SELECT` methods, and
* `->` - methods that execute `RETURNING` statements and provide access to returned data.

For `SELECT` statements (`?`) like:

```sql
-- name: get_loaned_books?
-- param: user_id: &str
SELECT book_title FROM library WHERE loaned_to = :user_id;
```

The method with the following signature is generated:

```rust , ignore
fn get_loaned_books<F>(
    &self,
    user_id: &str,
    row_callback: F
) -> -> rusqlite::Result<()>
where F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<()>;
```

For non-select statements (`!`) - INSERT, UPDATE, DELETE, etc. - like:

```sql
-- name: loan_books!
-- param: user_id: &str
-- param: book_ids: i32
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_id IN (:book_ids);
```

The method with the following signature is generated:

```rust , ignore
fn loan_books(
    &self,
    user_id: &str,
    book_ids: &[i32]
) -> rusqlite::Result<usize>;
```

For DELETE, INSERT, and UPDATE statements that return data via `RETURNING` clause (`->`) like:

```sql
-- name: add_new_book->
-- param: isbn: &str
-- param: book_title: &str
INSERT INTO library (isbn, book_title)
VALUES (:isbn, :book_title)
RETURNING book_id;
```

The method with the following signature is generated:

```rust , ignore
fn add_new_book<F,R>(
    &self,
    isbn: &str,
    book_title: &str,
    row_callback: F
) -> rusqlite::Result<R>
where F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<R>;
```
*/
#[macro_export]
macro_rules! impl_sql {
    ( $sql_name:ident = $( { $kind:tt $name:ident ($($variant:tt $param:ident $ptype:tt)*) $doc:literal $s:tt $( $text:tt )+ } ),+ ) => {
        trait $sql_name {
            $( $crate::decl_method!{ $kind $name $doc () () $($param $variant $ptype)* } )+
        }
        impl $sql_name for rusqlite::Connection {
            $( $crate::impl_method!{ $kind $name () () ($($param $variant $ptype)*) => ($($variant $param)*) $($text)+ } )+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! decl_method {
    ( ? $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<$($gen_type : rusqlite::ToSql ,)* F>(&self $($fn_params)* , row_cb: F) -> rusqlite::Result<()>
        where F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<()>;
    };
    ( ! $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<$($gen_type : rusqlite::ToSql),*>(&self $($fn_params)*) -> rusqlite::Result<usize>;
    };
    ( -> $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<$($gen_type : rusqlite::ToSql ,)* F, R>(&self $($fn_params)* , row_cb: F) -> rusqlite::Result<R>
        where F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<R>;
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : _ $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)*)
            ($($fn_params)* , $param : impl rusqlite::ToSql)
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : ($ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)*)
            ($($fn_params)* , $param : $ptype)
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident # [$gtype:ident] $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)* $gtype)
            ($($fn_params)* , $param : & [ $gtype ] )
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident # ($ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)*)
            ($($fn_params)* , $param : & [ $ptype ] )
            $($tail)*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_method {
    ( ? $name:ident () () () => () $text:literal ) => {
        fn $name<F>(&self, row_cb: F) -> rusqlite::Result<()>
        where F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<()>
        {
            let mut stmt = self.prepare( $text )?;
            let mut rows = stmt.raw_query();
            while let Some(row) = rows.next()? {
                row_cb(row)?;
            }
            Ok(())
        }
    };
    ( ? $name:ident () ($($fn_params:tt)+) () => ( $(: $param:ident)+ ) $($text:tt)+) => {
        fn $name<F>(&self $($fn_params)+ , row_cb: F) -> rusqlite::Result<()>
        where F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<()>
        {
            let mut stmt = self.prepare( $crate::sql_literal!( $($param)+ => $($text)+ ) )?;
            $crate::bind_args!($($param)+ => stmt 1usize);
            let mut rows = stmt.raw_query();
            while let Some(row) = rows.next()? {
                row_cb(row)?;
            }
            Ok(())
        }
    };
    ( ? $name:ident ($($gen_type:ident)*) ($($fn_params:tt)+) () => ($($pv:tt $param:ident)+) $($text:tt)+) => {
        fn $name<$($gen_type : rusqlite::ToSql ,)* F>(&self $($fn_params)+, row_cb: F) -> rusqlite::Result<()>
        where F: Fn(&rusqlite::Row<'_>) -> rusqlite::Result<()>
        {
            let mut sql = String::with_capacity($crate::sql_len!($($text)+));
            let mut args = Vec::<&dyn rusqlite::ToSql>::with_capacity($crate::num_args!($($pv $param)+));
            let mut i = 0;
            $crate::dynamic_sql!(sql args i $($text)+);
            let mut stmt = self.prepare(&sql)?;
            let mut rows = stmt.query(args.as_slice())?;
            while let Some(row) = rows.next()? {
                row_cb(row)?;
            }
            Ok(())
        }
    };
    ( ! $name:ident () () () => () $text:literal ) => {
        fn $name(&self) -> rusqlite::Result<usize> {
            let mut stmt = self.prepare( $text )?;
            stmt.raw_execute()
        }
    };
    ( ! $name:ident () ($($fn_params:tt)+) () => ( $(: $param:ident)+ ) $($text:tt)+) => {
        fn $name(&self $($fn_params)+ ) -> rusqlite::Result<usize> {
            let mut stmt = self.prepare( $crate::sql_literal!( $($param)+ => $($text)+ ) )?;
            $crate::bind_args!($($param)+ => stmt 1usize);
            stmt.raw_execute()
        }
    };
    ( ! $name:ident ($($gen_type:ident)*) ($($fn_params:tt)+) () => ($($pv:tt $param:ident)+) $($text:tt)+) => {
        fn $name<$($gen_type : rusqlite::ToSql),*>(&self $($fn_params)+ ) -> rusqlite::Result<usize> {
            let mut sql = String::with_capacity($crate::sql_len!($($text)+));
            let mut args = Vec::<&dyn rusqlite::ToSql>::with_capacity($crate::num_args!($($pv $param)+));
            let mut i = 0;
            $crate::dynamic_sql!(sql args i $($text)+);
            let mut stmt = self.prepare(&sql)?;
            stmt.execute(args.as_slice())
        }
    };
    ( -> $name:ident () () () => () $text:literal ) => {
        fn $name<F,R>(&self, row_cb: F) -> rusqlite::Result<R>
        where F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<R>
        {
            let mut stmt = self.prepare( $text )?;
            let mut rows = stmt.raw_query();
            match rows.next()? {
                Some(row) => row_cb(row),
                _ => Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }
    };
    ( -> $name:ident () ($($fn_params:tt)+) () => ( $(: $param:ident)+ ) $($text:tt)+) => {
        fn $name<F,R>(&self $($fn_params)+ , row_cb: F) -> rusqlite::Result<R>
        where F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<R>
        {
            let mut stmt = self.prepare( $crate::sql_literal!( $($param)+ => $($text)+ ) )?;
            $crate::bind_args!($($param)+ => stmt 1usize);
            let mut rows = stmt.raw_query();
            match rows.next()? {
                Some(row) => row_cb(row),
                _ => Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }
    };
    ( -> $name:ident ($($gen_type:ident)*) ($($fn_params:tt)+) () => ($($pv:tt $param:ident)+) $($text:tt)+) => {
        fn $name<$($gen_type : rusqlite::ToSql ,)* F,R>(&self $($fn_params)+, row_cb: F) -> rusqlite::Result<R>
        where F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<R>
        {
            let mut sql = String::with_capacity($crate::sql_len!($($text)+));
            let mut args = Vec::<&dyn rusqlite::ToSql>::with_capacity($crate::num_args!($($pv $param)+));
            let mut i = 0;
            $crate::dynamic_sql!(sql args i $($text)+);
            let mut stmt = self.prepare(&sql)?;
            let mut rows = stmt.query(args.as_slice())?;
            match rows.next()? {
                Some(row) => row_cb(row),
                _ => Err(rusqlite::Error::QueryReturnedNoRows)
            }
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : _ $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)*)
            ($($fn_params)* , $param : impl rusqlite::ToSql)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : ($ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)*)
            ($($fn_params)* , $param : $ptype)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # [$gtype:ident] $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)* $gtype)
            ($($fn_params)* , $param : & [ $gtype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # ($ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)*)
            ($($fn_params)* , $param : & [ $ptype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! sql_literal {
    ($($name:ident)+ => $text:literal) => {
        $text
    };
    ($($name:ident)+ => $text:literal : $param:ident) => {
        std::concat!( $text, '$', include_sql::index_of!($param in [ $( $name ),+ ] + 1) )
    };
    ($($name:ident)+ => $text:literal : $param:ident $($tail:tt)+) => {
        std::concat!(
            $text, '$', include_sql::index_of!($param in [ $( $name ),+ ] + 1),
            $crate::sql_literal!($($name)+ => $($tail)+)
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_args {
    ($head:ident $($tail:ident)* => $stmt:ident $idx:expr) => {
        $stmt.raw_bind_parameter($idx, $head)?;
        $crate::bind_args!($($tail)* => $stmt $idx+1usize);
    };
    (=> $stmt:ident $idx:expr) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! num_args {
    () => { 0 };
    (: $head:ident $($tail:tt)*) => { 1 + $crate::num_args!($($tail)*) };
    (# $head:ident $($tail:tt)*) => { $head.len() + $crate::num_args!($($tail)*) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! sql_len {
    () => { 0 };
    ($text:literal $($tail:tt)*) => { $text.len() + $crate::sql_len!($($tail)*) };
    (: $head:ident $($tail:tt)*) => { 3 + $crate::sql_len!($($tail)*) };
    (# $head:ident $($tail:tt)*) => { $head.len() * 5 + $crate::sql_len!($($tail)*) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! dynamic_sql {
    ($stmt:ident $args:ident $i:ident) => {};
    ($stmt:ident $args:ident $i:ident $text:literal $($tail:tt)*) => {
        $stmt.push_str($text);
        $crate::dynamic_sql!($stmt $args $i $($tail)*);
    };
    ($stmt:ident $args:ident $i:ident : $param:ident $($tail:tt)*) => {
        $i += 1;
        $stmt.push_str(&format!("${}", $i));
        $args.push(&$param);
        $crate::dynamic_sql!($stmt $args $i $($tail)*);
    };
    ($stmt:ident $args:ident $i:ident # $param:ident $($tail:tt)*) => {
        let mut iter = $param.into_iter();
        if let Some(arg) = iter.next() {
            $i += 1;
            $stmt.push_str(&format!("${}", $i));
            $args.push(arg);
            while let Some(arg) = iter.next() {
                $i += 1;
                $stmt.push_str(&format!(", ${}", $i));
                $args.push(arg);
            }
        } else {
            $stmt.push_str("NULL");
        }
        $crate::dynamic_sql!($stmt $args $i $($tail)*);
    };
}
