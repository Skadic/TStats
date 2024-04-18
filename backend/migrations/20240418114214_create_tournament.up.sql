CREATE TYPE osu_mode AS ENUM('osu', 'taiko', 'catch', 'mania');
CREATE TABLE tournament (
    id SERIAL PRIMARY KEY UNIQUE NOT NULL CHECK(id > 0),
    name VARCHAR(30) NOT NULL UNIQUE,
    shorthand VARCHAR(8) NOT NULL,
    "format" SMALLINT NOT NULL CHECK (format >= 0),
    bws BOOLEAN NOT NULL,
    mode osu_mode NOT NULL
);
CREATE INDEX ix_tournament_name ON tournament (name);