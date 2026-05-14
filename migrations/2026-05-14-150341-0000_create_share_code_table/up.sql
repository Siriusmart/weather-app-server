-- Your SQL goes here
CREATE TABLE share_codes (
    share_code          TEXT PRIMARY KEY NOT NULL,
    metoffice_key       TEXT UNIQUE NOT NULL
);
