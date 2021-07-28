CREATE TABLE categories (
    category_id SERIAL PRIMARY KEY,
    category_name VARCHAR NOT NULL,
    category_color VARCHAR
);

ALTER TABLE things ADD COLUMN category_id INT;

ALTER TABLE things ADD CONSTRAINT fk_category FOREIGN KEY (category_id) REFERENCES categories (category_id);