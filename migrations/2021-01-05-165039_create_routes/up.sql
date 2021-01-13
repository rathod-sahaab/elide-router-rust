CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE routes (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    slug UNIQUE VARCHAR NOT NULL,
    target VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 't'
);
