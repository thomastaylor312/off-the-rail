-- Run this with `cat testdata.sql | sqlite3 $HOME/Library/Application\ Support/off-the-rail/database`

INSERT INTO shows (name, start_date, location, entry_deadline) VALUES ('2023 Summer Show', '2023-06-01T00:00:00Z', 'The Ranch', '2023-05-01T00:00:00Z');
INSERT INTO shows (name, start_date, location, entry_deadline) VALUES ('2023 Fall Show', '2023-09-01T00:00:00Z', 'Salina', '2023-08-01T00:00:00Z');
INSERT INTO shows (name, start_date, entry_deadline) VALUES ('2023 Winter Show', '2023-12-01T00:00:00Z', '2023-11-01T00:00:00Z');

INSERT INTO riders (name, email, membership_date, phone, address) VALUES ('John Wayne', 'wayne@foo.com', '2023-01-01T00:00:00Z', '123-456-7890', '1234 Main St');
INSERT INTO riders (name, email, membership_date, phone, address) VALUES ('Harry Morgan', 'dr.potter@4077.com', '2023-01-01T00:00:00Z', '123-456-7890', '1234 Main St, Uijeongbu, South Korea');
INSERT INTO riders (name, email, phone, address) VALUES ('Alan Alda', 'hawkeye@4077.com', '123-456-7890', '1234 Main St, Uijeongbu, South Korea');
