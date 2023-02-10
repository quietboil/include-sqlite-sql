-- name: insert_new_quote !
-- param: author: &str
-- param: quote: &str
INSERT INTO quotes (author, quote) VALUES (:author, :quote)
/

-- name: insert_new_quote_gen !
INSERT INTO quotes (author, quote) VALUES (:author, :quote)
/

-- name: get_quotes_by_author ?
-- param: author: &str
SELECT quote
  FROM quotes
 WHERE author = :author
/
