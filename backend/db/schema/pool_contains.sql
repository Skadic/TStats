DEFINE TABLE pool_contains SCHEMAFULL;

DEFINE FIELD in ON TABLE pool_contains TYPE record (stage);
DEFINE FIELD out ON TABLE pool_contains TYPE record (map);
-- The bracket is a 1-3 letter long descriptor of the bracket, like "NM", "HD", "TB" etc.
DEFINE FIELD bracket ON TABLE pool_contains TYPE string
    ASSERT $value != NONE AND $value = /[A-Z]{0,3}/;
DEFINE FIELD bracket_order ON TABLE pool_contains TYPE int
    ASSERT $value != NONE AND $value >= 0;

DEFINE INDEX unique_slot ON TABLE pool_contains COLUMNS in, bracket, bracket_order UNIQUE;