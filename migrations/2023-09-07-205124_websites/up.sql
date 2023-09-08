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
  hostname VARCHAR NOT NULL,
  scraped BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER websites_set_timestamp
BEFORE UPDATE ON websites
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_updated_timestamp();

-- trips table
CREATE TABLE trips (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

INSERT INTO trips (name) VALUES ('default');

CREATE TRIGGER trips_set_timestamp
BEFORE UPDATE ON trips
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_updated_timestamp();

-- bookmarks table
CREATE TABLE bookmarks (
  id SERIAL PRIMARY KEY,
  link VARCHAR NOT NULL,
  trip_id SERIAL REFERENCES trips(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER bookmarks_set_timestamp
BEFORE UPDATE ON bookmarks
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_updated_timestamp();

