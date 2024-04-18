CREATE TABLE pool_map (
    tournament_id SERIAL NOT NULL,
    stage_order SMALLINT NOT NULL,
    bracket_order SMALLINT NOT NULL,
    map_order SMALLINT NOT NULL CHECK (map_order >= 0),
    map_id BIGINT NOT NULL,
    PRIMARY KEY (
        tournament_id,
        stage_order,
        bracket_order,
        map_order
    ),
    FOREIGN KEY (tournament_id, stage_order, bracket_order) REFERENCES pool_bracket
);