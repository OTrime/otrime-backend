-- Add migration script here
create table questions(title varchar(250) NOT NULL,question varchar(4000) NOT NULL, id uuid PRIMARY KEY NOT NULL, by uuid NOT NULL, asked_on timestamp NOT NULL);