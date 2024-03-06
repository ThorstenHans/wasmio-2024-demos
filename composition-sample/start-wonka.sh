#! /bin/bash

echo "Starting Wonka Service"

pushd wonka-3rd-party-service
spin up --listen 127.0.0.1:3002
