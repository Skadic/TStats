CREATE TABLE score (
    player_id INT NOT NULL,
    tournament_id SERIAL NOT NULL,
    stage_order SMALLINT NOT NULL,
    bracket_order SMALLINT NOT NULL,
    map_order SMALLINT NOT NULL,
    match_id SERIAL NOT NULL REFERENCES match(id),
    score BIGINT NOT NULL CHECK (score >= 0),
    PRIMARY KEY (player_id, tournament_id, stage_order, bracket_order, map_order, match_id),
    FOREIGN KEY (tournament_id, stage_order, bracket_order, map_order) REFERENCES pool_map
);