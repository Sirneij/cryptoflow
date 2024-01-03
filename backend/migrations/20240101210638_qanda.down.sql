-- Add down migration script here
DROP TABLE IF EXISTS question_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS answers;
DROP TABLE IF EXISTS questions;
DROP FUNCTION IF EXISTS trigger_set_timestamp();