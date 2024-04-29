CREATE TABLE match_link (
    match_id SERIAL NOT NULL REFERENCES match(id),
    link_order SMALLSERIAL NOT NULL CHECK (link_order >= 0),
    osu_mp_id INT NOT NULL CHECK (osu_mp_id >= 0),
    PRIMARY KEY (match_id, link_order),
    UNIQUE (link_order)
);
