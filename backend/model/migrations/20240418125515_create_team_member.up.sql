CREATE TABLE team_member (
    team_id SERIAL NOT NULL REFERENCES team(id),
    user_id INT NOT NULL CHECK (user_id >= 0),
    PRIMARY KEY (team_id, user_id)
);