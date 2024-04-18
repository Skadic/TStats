-- Add down migration script here

DROP USER MAPPING FOR PUBLIC SERVER redis_server;
DROP VIEW map;
DROP FOREIGN TABLE raw_map;
DROP SERVER redis_server;
DROP EXTENSION redis_fdw;