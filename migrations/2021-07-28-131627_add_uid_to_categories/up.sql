ALTER TABLE categories ADD COLUMN user_id INT NOT NULL;

ALTER TABLE categories ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (user_id);