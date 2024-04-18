-- Add up migration script here
CREATE TABLE team (
    id SERIAL NOT NULL PRIMARY KEY,
    tournament_id SERIAL NOT NULL REFERENCES tournament(id),
    name VARCHAR(40) NOT NULL,
    UNIQUE (tournament_id, name)
);

CREATE INDEX ix_team_name ON team (name); 