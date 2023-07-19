DEFINE TABLE is_stage SCHEMAFULL;

DEFINE FIELD in ON TABLE is_stage TYPE record (tournament);
DEFINE FIELD out ON TABLE is_stage TYPE record (stage);

DEFINE INDEX id ON TABLE is_stage COLUMNS in, out UNIQUE;
