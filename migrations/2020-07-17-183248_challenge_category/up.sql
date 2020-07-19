-- Add category_id to `challenges` with a foreign key constraint
-- to `categories`

ALTER TABLE challenges
ADD category_id INTEGER NOT NULL;

ALTER TABLE challenges
ADD CONSTRAINT challenges_category_id_fkey
FOREIGN KEY (category_id) REFERENCES categories(id);