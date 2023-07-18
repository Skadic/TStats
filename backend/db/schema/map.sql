DEFINE TABLE map SCHEMAFULL;

DEFINE FIELD map_id ON TABLE map TYPE int
  ASSERT $value != NONE;
DEFINE FIELD stage ON TABLE map TYPE record (stage)
  ASSERT $value != NONE;
DEFINE FIELD bracket ON TABLE map TYPE string
    ASSERT $value != NONE AND $value = /[A-Z]{0,3}/;
DEFINE FIELD bracket_order ON TABLE map TYPE int
    ASSERT $value != NONE AND $value >= 0;

DEFINE INDEX id ON TABLE map COLUMNS stage, bracket, bracket_order UNIQUE;
