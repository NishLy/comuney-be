-- This file should undo anything in `up.sql`
DROP TABLE users;
DROP FUNCTION update_timestamp_function;
DROP TRIGGER update_timestamp ON users;