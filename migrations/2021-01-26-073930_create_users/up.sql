CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT 'f',
    active BOOLEAN NOT NULL DEFAULT 't',
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

-- To automatically refresh updated_at
CREATE OR REPLACE FUNCTION refresh_updated_at()
RETURNS TRIGGER AS $$
BEGIN
   IF row(NEW.*) IS DISTINCT FROM row(OLD.*) THEN
      NEW.updated_at = now(); 
      RETURN NEW;
   ELSE
      RETURN OLD;
   END IF;
END;
$$ language 'plpgsql';

CREATE TRIGGER refresh_user_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE  refresh_updated_at();

