-- Your SQL goes here
CREATE TABLE problems (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  grade INTEGER NOT NULL,
  rating INTEGER,
  creator INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('problems');

CREATE UNIQUE INDEX title_unique_idx ON problems (LOWER(title));