#!/bin/bash

docker run \
  --rm \
  --detach \
  --name postgres \
  --env POSTGRES_USER=username \
  --env POSTGRES_PASSWORD=password \
  --publish 127.0.0.1:8001:5432 \
  postgres

DATABASE_URL=postgres://username:password@localhost:8001/postgres cargo test -- --test-threads=1

docker stop postgres