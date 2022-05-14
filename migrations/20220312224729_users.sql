-- Add migration script here
CREATE TABLE users(id uuid PRIMARY KEY NOT NULL, email varchar(100) NOT NULL UNIQUE, password text NOT NULL, name varchar(100) NOT NULL, joined timestamp NOT NULL);