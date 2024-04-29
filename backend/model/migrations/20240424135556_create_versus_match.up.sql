-- Add up migration script here
CREATE TABLE versus_match (
    match_id SERIAL NOT NULL PRIMARY KEY,
    team_red SERIAL NOT NULL REFERENCES team(id),
    team_blue SERIAL NOT NULL REFERENCES team(id),
    score_red SMALLINT CHECK (score_red >= 0),
    score_blue SMALLINT CHECK (score_blue >= 0),
    "match_type" match_type NOT NULL CHECK ("match_type" = 'versus_match'),
    FOREIGN KEY ("match_id", "match_type") REFERENCES match("id", "match_type")
);