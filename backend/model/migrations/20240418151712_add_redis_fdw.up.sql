-- Add up migration script here
CREATE EXTENSION redis_fdw;
CREATE SERVER redis_server FOREIGN DATA WRAPPER redis_fdw OPTIONS (address 'tstats-redis', port '6379');
CREATE USER MAPPING FOR PUBLIC SERVER redis_server;