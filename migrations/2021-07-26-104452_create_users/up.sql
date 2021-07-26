CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  user_email VARCHAR NOT NULL,
  user_pwd_hash BYTEA NOT NULL,
  user_salt VARCHAR NOT NULL
);