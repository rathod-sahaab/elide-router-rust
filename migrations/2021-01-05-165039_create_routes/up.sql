-- Your SQL goes here
CREATE TABLE routes (
    uuid UUID PRIMARY KEY,
    slug VARCHAR NOT NULL,
    target VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 'f'
);
