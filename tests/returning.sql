-- name: insert_new_quote ->
-- param: author: &str
-- param: quote: &str
INSERT INTO quotes (author, quote) VALUES (:author, :quote) RETURNING id
/

-- name: insert_new_quote_gen ->
INSERT INTO quotes (author, quote) VALUES (:author, :quote) RETURNING id
/

-- name: get_quote_by_id ?
-- param: id: i32
SELECT author, quote
  FROM quotes
 WHERE id = :id
/
