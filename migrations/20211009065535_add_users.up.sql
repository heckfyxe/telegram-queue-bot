CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    nickname VARCHAR(50) UNIQUE NOT NULL,
    full_name VARCHAR(50)
);