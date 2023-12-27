#!/bin/bash
source .env

if !(echo "$DATABASE_URL" | grep -E "localhost|127.0.0.1|0.0.0.0" > /dev/null) && [ ! "$FORCE" == "true" ]; then 
    echo DATABASE_URL is not localhost, skipping database recreation
    echo $DATABASE_URL
    exit 1
fi

sqlx database drop
sqlx database create
sqlx migrate run