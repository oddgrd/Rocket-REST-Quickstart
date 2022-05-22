#!/bin/bash

docker run \
  --rm \
  --detach \
  --name postgres \
  --env POSTGRES_USER=username \
  --env POSTGRES_PASSWORD=password \
  --publish 127.0.0.1:8001:5432 \
  postgres

