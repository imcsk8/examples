-- Your SQL goes here

CREATE TABLE IF NOT EXISTS Event (
    Id UUID,
    Name TEXT,
    Venue TEXT,
    Address TEXT,
    ContactName VARCHAR(255),
    Starts_at TIMESTAMP NOT NULL DEFAULT NOW(),
    Ends_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY(Id)
)

