-- Create auth table.
CREATE TABLE IF NOT EXISTS auth
(
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role TEXT NOT NULL
);