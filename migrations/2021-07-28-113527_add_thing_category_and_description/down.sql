ALTER TABLE things DROP CONSTRAINT fk_category;
ALTER TABLE things DROP COLUMN category_id;

DROP TABLE categories;