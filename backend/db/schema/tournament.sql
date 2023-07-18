DEFINE TABLE tournament SCHEMAFULL;

DEFINE FIELD name ON TABLE tournament TYPE string
  ASSERT $value != NONE;
DEFINE FIELD shorthand ON TABLE tournament TYPE string
  ASSERT $value != NONE;
DEFINE FIELD format ON TABLE tournament TYPE object
  ASSERT $value != NONE;
DEFINE FIELD rank_range ON TABLE tournament TYPE object
  VALUE $value OR NULL;
DEFINE FIELD bws ON TABLE tournament TYPE bool 
  VALUE $value OR false;
DEFINE FIELD country_restriction ON TABLE tournament TYPE array
  VALUE $value OR NULL;
DEFINE FIELD country_restriction.* ON TABLE tournament TYPE string
  ASSERT $value != NONE AND $value = /[A-Z]{3}/
  VALUE $value;

DEFINE INDEX id ON TABLE tournament COLUMNS name UNIQUE;