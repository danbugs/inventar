ALTER TABLE users ADD COLUMN verification_code VARCHAR;
UPDATE users SET verification_code='pre-verification';
ALTER TABLE users ALTER COLUMN verification_code SET NOT NULL;