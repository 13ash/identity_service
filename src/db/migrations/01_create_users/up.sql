-- Your SQL goes here
CREATE TABLE users (
                       id UUID NOT NULL PRIMARY KEY,
                       email VARCHAR(100) NOT NULL,
                       username VARCHAR(100) NOT NULL,
                       hash VARCHAR(122) NOT NULL, --argon hash password
                       first_name VARCHAR(100) NOT NULL,
                       last_name  VARCHAR(100) NOT NULL,
                       created_at TIMESTAMP NOT NULL
);