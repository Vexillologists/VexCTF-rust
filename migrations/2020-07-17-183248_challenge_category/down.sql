ALTER TABLE challenges
DROP CONSTRAINT challenges_category_id_fkey;

ALTER TABLE challenges
DROP COLUMN category_id;