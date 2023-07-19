DEFINE TABLE stage SCHEMAFULL;

DEFINE FIELD order ON TABLE stage TYPE int
  ASSERT $value != NONE AND $value >= 0;
DEFINE FIELD name ON TABLE stage TYPE string
  ASSERT $value != NONE;
DEFINE FIELD tournament ON TABLE stage TYPE record (tournament)
  ASSERT $value != NONE;
DEFINE FIELD pool_brackets ON TABLE stage TYPE array
  ASSERT $value != NONE;
DEFINE FIELD pool_brackets.* ON TABLE stage TYPE string
  ASSERT $value != NONE AND $value = /[A-Z]{1,3}/;

DEFINE INDEX id ON TABLE stage COLUMNS tournament, order UNIQUE;