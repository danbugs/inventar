ALTER TABLE things ADD COLUMN user_id INT NOT NULL;

ALTER TABLE things ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (user_id);