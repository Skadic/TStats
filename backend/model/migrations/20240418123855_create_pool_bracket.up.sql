CREATE TABLE pool_bracket (
    tournament_id SERIAL NOT NULL,
    stage_order SMALLINT NOT NULL,
    bracket_order SMALLINT NOT NULL CHECK (bracket_order >= 0),
    name VARCHAR(10) NOT NULL,
    PRIMARY KEY (tournament_id, stage_order, bracket_order),
    FOREIGN KEY (tournament_id, stage_order) REFERENCES stage,
    UNIQUE (tournament_id, stage_order, name)
);