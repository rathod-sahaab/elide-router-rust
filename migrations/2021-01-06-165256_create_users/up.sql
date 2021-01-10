-- Your SQL goes here
CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    display_name VARCHAR NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    email VARCHAR(254) NOT NULL UNIQUE
);
