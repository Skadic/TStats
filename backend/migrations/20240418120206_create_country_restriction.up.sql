CREATE TABLE country_restriction (
    tournament_id SERIAL NOT NULL REFERENCES tournament(id),
    country_code CHAR(2) NOT NULL,
    PRIMARY KEY (tournament_id, country_code)
)