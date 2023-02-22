CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    is_grower BOOLEAN NOT NULL,
    password_hash VARCHAR(255) NOT NULL
);