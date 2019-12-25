-- Your SQL goes here
--I would want to eventually encrypt and decrypt user data
CREATE TABLE users (
    ID SERIAL primary key not null,
    username VARCHAR(20) UNIQUE,
    firstName VARCHAR(25) CHECK (firstName NOT LIKE '%[^a-z]%') not null,
    lastName VARCHAR(25) CHECK (lastName NOT LIKE '%[^a-z]%') NULL
    -- passwordHash; eventually secure the user database
);