-- Create a table for categories
-- The `category` for `challenges` 
-- and its foreign key will be added
-- in a different migration

CREATE TABLE categories (
  id SERIAL PRIMARY KEY NOT NULL,
  category_name CITEXT UNIQUE NOT NULL,
  category_description TEXT NOT NULL
);