FROM alpine AS build

RUN apk add git make musl-dev hiredis-dev postgresql-dev
RUN git clone https://github.com/pg-redis-fdw/redis_fdw --branch REL_16_STABLE
RUN cd redis_fdw && make USE_PGXS=1

FROM postgres:16-alpine
COPY --from=build redis_fdw redis_fdw/.
RUN pg_config --pgxs
RUN apk add make hiredis-dev
RUN cd redis_fdw && make USE_PGXS=1 with_llvm=no install

STOPSIGNAL SIGINT
EXPOSE 5432
CMD [ "postgres" ]
