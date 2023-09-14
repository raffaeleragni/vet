#!/usr/bin/env bash
echo "request"

docker compose down
docker compose up -d

while [ -z "`docker compose logs wiremock | grep '8080' `" ]; do
    sleep 0.1
    echo -n "."
done

$VET

docker compose down
