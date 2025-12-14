#!/usr/bin/env sh
podman build -t chatapp-db ./db
podman run --replace -d \
    --name chatapp-db \
    -p 5432:5432 \
    chatapp-db
