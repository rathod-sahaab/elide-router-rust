CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE routes (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    slug VARCHAR NOT NULL UNIQUE ,
    target VARCHAR NOT NULL,
    creator_id UUID,
    active BOOLEAN NOT NULL DEFAULT 't',
    active_from TIMESTAMP, -- time based activation
    active_till TIMESTAMP, -- time based deactivation
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    CONSTRAINT fk_user
        FOREIGN KEY(creator_id) 
        REFERENCES users(id)
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

CREATE TRIGGER refresh_route_updated_at BEFORE UPDATE ON routes FOR EACH ROW EXECUTE PROCEDURE  refresh_updated_at();
