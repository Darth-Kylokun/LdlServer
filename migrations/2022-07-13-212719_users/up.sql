-- Your SQL goes here
CREATE TABLE users (
    uuid VARCHAR(36) PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL
)