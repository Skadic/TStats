CREATE TABLE stage (
    tournament_id SERIAL NOT NULL REFERENCES tournament(id),
    stage_order SMALLINT NOT NULL CHECK (stage_order >= 0),
    name VARCHAR(10) NOT NULL,
    best_of SMALLINT NOT NULL CHECK(best_of >= 0),
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    PRIMARY KEY (tournament_id, stage_order),
    UNIQUE (tournament_id, name),
    CHECK (start_date <= end_date)
);