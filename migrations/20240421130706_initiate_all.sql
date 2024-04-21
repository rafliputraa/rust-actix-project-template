-- Add migration script here
-- Table definition for User
CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       first_name VARCHAR(100) NOT NULL,
                       last_name VARCHAR(100) NOT NULL
);

-- Table definition for Article
CREATE TABLE articles (
                          id SERIAL PRIMARY KEY,
                          title VARCHAR(255) NOT NULL,
                          content TEXT NOT NULL,
                          created_by INTEGER NOT NULL REFERENCES users(id)
);