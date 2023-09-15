#!/usr/bin/env bash
echo "request"

docker compose down
docker compose up -d

while [ -z "`docker compose logs kafka | grep 'Awaiting socket connections' `" ]; do
    sleep 0.1
    echo -n "."
done

$VET

docker compose down
