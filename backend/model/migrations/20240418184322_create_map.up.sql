CREATE FOREIGN TABLE raw_map (key text, val json) SERVER redis_server OPTIONS (database '0', tablekeyprefix 'map:');
CREATE VIEW map AS
SELECT CAST(SPLIT_PART(key, ':', 2) AS INT) as map_id,
    val->>'artistName' AS artist_name,
    val->>'name' AS name,
    val->>'diffName' AS diff_name,
    val->>'setId' AS set_id,
    CAST(val->>'creator' AS JSON) AS creator,
    CAST(val->>'difficulty' AS JSON) AS difficulty
FROM raw_map;
