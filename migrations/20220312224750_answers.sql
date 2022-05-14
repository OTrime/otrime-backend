-- Add migration script here
create table answers(answer text NOT NULL, id uuid NOT NULL PRIMARY KEY, for_question uuid NOT NULL, by uuid NOT NULL, answered_on timestamp NOT NULL);