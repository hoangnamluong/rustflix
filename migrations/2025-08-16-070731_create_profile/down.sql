-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS profile;
DROP TYPE IF EXISTS maturity_rating;
DROP FUNCTION IF EXISTS set_default_language();