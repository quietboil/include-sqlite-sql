-- name: count_quotes?
SELECT Count(*) FROM quotes
/

-- name: get_quotes_longer_than ?
-- param: min_len: i32 - min quote len
SELECT author, quote
  FROM quotes
 WHERE Length(quote) >= :min_len
 ORDER BY Length(quote)
/

-- name: get_quotes_within_range ?
-- expecting generic parameters
SELECT author, quote
  FROM quotes
 WHERE Length(quote) BETWEEN :min_len AND :max_length
 ORDER BY Length(quote)
/
