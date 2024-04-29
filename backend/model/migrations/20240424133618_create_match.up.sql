CREATE TYPE match_type AS ENUM('qualifier', 'versus_match');
CREATE TABLE match (
    id SERIAL NOT NULL PRIMARY KEY,
    tournament_id SERIAL NOT NULL,
    stage_order SMALLINT NOT NULL,
    "date" timestamp NOT NULL,
    "match_type" match_type NOT NULL,
    FOREIGN KEY (tournament_id, stage_order) REFERENCES stage,
    UNIQUE (id, match_type)
);