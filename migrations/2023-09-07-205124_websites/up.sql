-- Your SQL goes here

CREATE OR REPLACE FUNCTION trigger_set_updated_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- websites tables
CREATE TABLE websites (
  id SERIAL PRIMARY KEY,
  hostname TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TRIGGER websites_set_timestamp
BEFORE UPDATE ON websites
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_updated_timestamp();

-- trips table
CREATE TABLE trips (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()

);

CREATE TRIGGER trips_set_timestamp
BEFORE UPDATE ON trips
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_updated_timestamp();

-- bookmarks table
CREATE TABLE bookmarks (
  id SERIAL PRIMARY KEY,
  link TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TRIGGER bookmarks_set_timestamp
BEFORE UPDATE ON bookmarks
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_updated_timestamp();

