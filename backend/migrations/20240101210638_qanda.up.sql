-- Add up migration script here
-- Trigger function to update the timestamp on the 'questions' table
CREATE OR REPLACE FUNCTION update_questions_timestamp() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- Trigger function to update the timestamp on the 'answers' table
CREATE OR REPLACE FUNCTION update_answers_timestamp() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- Questions table
CREATE TABLE IF NOT EXISTS questions (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    raw_content TEXT NOT NULL,
    author UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS questions_index_title ON questions (title, slug);
CREATE TRIGGER update_questions_timestamp BEFORE
UPDATE ON questions FOR EACH ROW EXECUTE PROCEDURE update_questions_timestamp();
-- Answers table
CREATE TABLE IF NOT EXISTS answers (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    raw_content TEXT NOT NULL,
    author UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    question UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER update_answers_timestamp BEFORE
UPDATE ON answers FOR EACH ROW EXECUTE PROCEDURE update_answers_timestamp();
-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    symbol VARCHAR (255) NOT NULL
);
CREATE INDEX IF NOT EXISTS tags_index_name ON tags (name);
CREATE INDEX IF NOT EXISTS tags_index_symbol ON tags (symbol);
-- Question tags table
CREATE TABLE IF NOT EXISTS question_tags (
    question UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    tag VARCHAR(255) NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (question, tag)
);