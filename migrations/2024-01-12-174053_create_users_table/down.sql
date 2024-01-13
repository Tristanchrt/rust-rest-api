-- This file should undo anything in `up.sql`
-- Drop users table
DROP TABLE IF EXISTS users;
ALTER TABLE posts DROP FOREIGN KEY IF EXISTS posts_user_id_fk;

-- Drop posts table
DROP TABLE IF EXISTS posts;
