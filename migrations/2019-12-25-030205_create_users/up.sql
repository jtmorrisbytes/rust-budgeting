-- Your SQL goes here
--I would want to eventually encrypt and decrypt user data
CREATE TABLE users (
    id INT primary key not null,
    username VARCHAR(20) UNIQUE NOT NULL,
    firstName VARCHAR(25) CHECK (firstName NOT LIKE '%[^a-z]%') not null,
    lastName VARCHAR(25)  CHECK (lastName NOT LIKE '%[^a-z]%') NULL
    -- passwordHash; eventually secure the user database
);
INSERT INTO Users VALUES
    (0, 'jthec', 'Jordan', 'Morris'),
    (1, 'jdoe', 'jane', 'doe')
;