CREATE TABLE qualifier_run (
    match_id SERIAL NOT NULL PRIMARY KEY,
    team_id SERIAL NOT NULL REFERENCES team(id),
    "match_type" match_type NOT NULL CHECK ("match_type" = 'qualifier'),
    FOREIGN KEY ("match_id", "match_type") REFERENCES match("id", "match_type"),
    UNIQUE (match_id, team_id)
);