#! /bin/bash

echo "Starting ACME Product"

pushd acme-product
spin up --listen 127.0.0.1:3000 --sqlite @migration.sql
