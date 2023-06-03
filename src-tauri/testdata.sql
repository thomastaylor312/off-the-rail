-- Run this with `cat testdata.sql | sqlite3 $HOME/Library/Application\ Support/off-the-rail/database`

INSERT INTO shows (name, start_date, location, entry_deadline) VALUES ('2023 Summer Show', '2023-06-01T00:00:00Z', 'The Ranch', '2023-05-01T00:00:00Z');
INSERT INTO shows (name, start_date, location, entry_deadline) VALUES ('2023 Fall Show', '2023-09-01T00:00:00Z', 'Salina', '2023-08-01T00:00:00Z');
INSERT INTO shows (name, start_date, entry_deadline) VALUES ('2023 Winter Show', '2023-12-01T00:00:00Z', '2023-11-01T00:00:00Z');
