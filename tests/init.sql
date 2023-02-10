-- name: create_test_table !
CREATE TABLE quotes (
    id      INTEGER PRIMARY KEY, 
    author  TEXT, 
    quote   TEXT
)
/

-- name: insert_test_quotes &
BEGIN;
INSERT INTO quotes (id, author, quote) VALUES (1, 'John Keats', 'Impossible is for the unwilling.');
INSERT INTO quotes (id, author, quote) VALUES (2, 'Thomas Carlyle', 'No pressure, no diamonds.');
INSERT INTO quotes (id, author, quote) VALUES (3, 'Theodore Roosevelt', 'Believe you can and you’re halfway there.');
INSERT INTO quotes (id, author, quote) VALUES (4, 'George Eliot', 'It is never too late to be what you might have been.');
INSERT INTO quotes (id, author, quote) VALUES (5, 'Confucius', 'Wherever you go, go with all your heart.');
INSERT INTO quotes (id, author, quote) VALUES (6, 'Albert Einstein', 'Life is like riding a bicycle. To keep your balance, you must keep moving.');
INSERT INTO quotes (id, author, quote) VALUES (7, 'Vincent Van Gogh', 'Great things are done by a series of small things brought together.');
INSERT INTO quotes (id, author, quote) VALUES (8, 'C. S. Lewis', 'You don’t have a soul. You are a soul. You have a body.');
INSERT INTO quotes (id, author, quote) VALUES (9, 'George Orwell', 'Freedom is the right to tell people what they do not want to hear.');
INSERT INTO quotes (id, author, quote) VALUES (10, 'Laozi', 'The journey of a thousand miles begins with a single step.');
COMMIT;
/
