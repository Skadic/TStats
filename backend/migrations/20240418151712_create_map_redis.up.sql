-- Add up migration script here
CREATE EXTENSION redis_fdw;
CREATE SERVER redis_server FOREIGN DATA WRAPPER redis_fdw OPTIONS (address 'tstats-redis', port '6379');
CREATE FOREIGN TABLE raw_map (key text, val json) SERVER redis_server OPTIONS (database '0', tablekeyprefix 'map:');
CREATE VIEW map AS
SELECT CAST(SPLIT_PART(key, ':', 2) AS INT) as map_id,
    val as data
FROM raw_map;
CREATE USER MAPPING FOR PUBLIC SERVER redis_server;