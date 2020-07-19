-- We're going to use the citext extension as
-- it provides us with a more sensible method for
-- uniqueness in usernames and emails

CREATE EXTENSION citext;