-- We want this table schemaless because format and rank range can have multiple variants
DEFINE TABLE tournament SCHEMALESS;

-- The tournament's full name
DEFINE FIELD name ON TABLE tournament TYPE string
  ASSERT $value != NONE;
-- The tournament's short name like "OWC" for "Osu World Cup"
DEFINE FIELD shorthand ON TABLE tournament TYPE string
  ASSERT $value != NONE;
-- The tournament format, (like 3v3 versus)
DEFINE FIELD format ON TABLE tournament TYPE object
  ASSERT $value != NONE;
-- Rank ranges are optional for open rank tournaments
DEFINE FIELD rank_range ON TABLE tournament TYPE object
  VALUE $value OR NULL;
-- Whether the tournament uses badge weighting
DEFINE FIELD bws ON TABLE tournament TYPE bool
  VALUE $value OR false;
-- Whether the tournament is restricted to certain countries
DEFINE FIELD country_restriction ON TABLE tournament TYPE array
  VALUE $value OR NULL;
-- Use ISO 3166-1 alpha-2 country codes
DEFINE FIELD country_restriction.* ON TABLE tournament TYPE string
  ASSERT $value != NONE AND $value = /[A-Z]{2}/
  VALUE $value;

DEFINE INDEX id ON TABLE tournament COLUMNS name UNIQUE;