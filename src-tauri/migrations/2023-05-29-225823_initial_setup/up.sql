CREATE TABLE horses (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name VARCHAR(255) NOT NULL
);

CREATE TABLE shows (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  start_date DATETIME NOT NULL,
  location TEXT,
  entry_deadline DATETIME
);

CREATE TABLE riders (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    membership_date DATETIME,
    birthday DATETIME,
    phone VARCHAR(255),
    address TEXT,
    person_responsible VARCHAR(255)
);

CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    back_number INTEGER NOT NULL,
    horse_id INTEGER NOT NULL,
    show_id INTEGER NOT NULL,
    FOREIGN KEY (horse_id) REFERENCES horses(id),
    FOREIGN KEY (show_id) REFERENCES shows(id)
    UNIQUE (back_number, horse_id, show_id)
);

CREATE TABLE divisions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL
);

INSERT INTO divisions (name) VALUES ('Amateur');
INSERT INTO divisions (name) VALUES ('Open');
INSERT INTO divisions (name) VALUES ('Youth');
INSERT INTO divisions (name) VALUES ('Novice');
INSERT INTO divisions (name) VALUES ('Green Horse');
INSERT INTO divisions (name) VALUES ('Senior');

CREATE TABLE classes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    division_id INTEGER NOT NULL,
    -- If I _really_ wanted to get fancy, I could just have this be a class and then do some sort of join between the two tables
    FOREIGN KEY (division_id) REFERENCES divisions(id)
);

INSERT INTO classes (name, division_id) VALUES ('Cow Work', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Boxing', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Fence Work', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Western Pleasure', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Western Riding', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Working Cow Horse', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Riding', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Trail', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Reining', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Cutting', (SELECT id FROM divisions WHERE name = 'Amateur'));
INSERT INTO classes (name, division_id) VALUES ('Cow Work', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Boxing', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Fence Work', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Western Pleasure', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Western Riding', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Working Cow Horse', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Riding', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Trail', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Reining', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Cutting', (SELECT id FROM divisions WHERE name = 'Open'));
INSERT INTO classes (name, division_id) VALUES ('Cow Work', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Boxing', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Fence Work', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Western Pleasure', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Western Riding', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Working Cow Horse', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Riding', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Trail', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Reining', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Cutting', (SELECT id FROM divisions WHERE name = 'Youth'));
INSERT INTO classes (name, division_id) VALUES ('Cow Work', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Boxing', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Fence Work', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Western Pleasure', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Western Riding', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Working Cow Horse', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Riding', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Trail', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Reining', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Cutting', (SELECT id FROM divisions WHERE name = 'Novice'));
INSERT INTO classes (name, division_id) VALUES ('Cow Work', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Boxing', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Fence Work', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Western Pleasure', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Western Riding', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Working Cow Horse', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Riding', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Trail', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Reining', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Cutting', (SELECT id FROM divisions WHERE name = 'Green Horse'));
INSERT INTO classes (name, division_id) VALUES ('Cow Work', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Boxing', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Fence Work', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Western Pleasure', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Western Riding', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Working Cow Horse', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Riding', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Trail', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Reining', (SELECT id FROM divisions WHERE name = 'Senior'));
INSERT INTO classes (name, division_id) VALUES ('Ranch Cutting', (SELECT id FROM divisions WHERE name = 'Senior'));

CREATE TABLE results (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    entry_id INTEGER NOT NULL,
    class_id INTEGER NOT NULL,
    placing INTEGER,
    payout REAL,
    FOREIGN KEY (entry_id) REFERENCES entries(id),
    FOREIGN KEY (class_id) REFERENCES classes(id)
);

CREATE TABLE scores (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    result_id INTEGER NOT NULL,
    starting_score INTEGER NOT NULL,
    content_score INTEGER,
    penalty INTEGER,
    off_pattern BOOLEAN,
    FOREIGN KEY (result_id) REFERENCES results(id)
);
