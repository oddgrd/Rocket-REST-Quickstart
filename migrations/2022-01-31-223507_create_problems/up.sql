-- Your SQL goes here
CREATE TABLE problems (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  grade INTEGER NOT NULL,
  rating INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX title_unique_idx ON problems (LOWER(title));