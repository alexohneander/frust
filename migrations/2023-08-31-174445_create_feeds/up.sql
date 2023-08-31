-- Your SQL goes here
CREATE TABLE feeds (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  feedtype VARCHAR NOT NULL DEFAULT 'Unknown'
)