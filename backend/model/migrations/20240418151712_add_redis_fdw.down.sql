-- Add down migration script here

DROP USER MAPPING FOR PUBLIC SERVER redis_server;
DROP SERVER redis_server;
DROP EXTENSION redis_fdw;