#! /bin/bash

echo "Starting Cyberdyne Service"

pushd cyberdyne-3rd-party-service
spin up --listen 127.0.0.1:3001 --sqlite @migration.sql
