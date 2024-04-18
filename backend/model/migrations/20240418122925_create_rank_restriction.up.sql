-- Add up migration script here
CREATE TABLE rank_restriction (
    tournament_id SERIAL NOT NULL REFERENCES tournament(id),
    tier SMALLINT NOT NULL,
    "min" INT NOT NULL,
    "max" INT NOT NULL,
    PRIMARY KEY (tournament_id, tier),
    CHECK (min < max)
);