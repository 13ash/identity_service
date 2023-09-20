-- Your SQL goes here
CREATE TABLE users (
                       id UUID NOT NULL PRIMARY KEY,
                       email VARCHAR(100) NOT NULL UNIQUE,
                       username VARCHAR(100) NOT NULL UNIQUE,
                       hash VARCHAR NOT NULL, --argon hash password
                       random_salt VARCHAR NOT NULL,
                       first_name VARCHAR(100) NOT NULL,
                       last_name  VARCHAR(100) NOT NULL,
                       created_at TIMESTAMP NOT NULL
);