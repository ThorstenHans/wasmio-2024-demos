.PHONY register-wonka:
register-wonka:
	curl -X DELETE http://localhost:3000/registrations
	curl -X POST -H 'Content-Type: application/json' -d '{"url": "http://localhost:3002/target", "event": "customer_created"}' http://localhost:3000/registrations

.PHONY fire-webhook:
	curl -X POST http://localhost:3000/fire

.PHONY build-all: build-hmac build-acme build-wonka build-cyberdyne

.PHONY build-hmac:
build-hmac:
	pushd hmac;\
	make build

.PHONY build-acme:
build-acme:
	pushd acme;\
	make build

.PHONY build-wonka:
build-wonka:
	pushd wonka;\
	make build

.PHONY build-cyberdyne:
build-cyberdyne:
	pushd cyberdyne;\
	make build

.PHONY start-acme:
start-acme:
	pushd acme;\
	spin up --listen 127.0.0.1:3000 --sqlite @migration.sql

.PHONY start-wonka:
start-wonka:
	pushd wonka;\
	spin up --listen 127.0.0.1:3002

.PHONY start-cyberdyne:
start-cyberdyne:
	pushd cyberdyne;\
	spin up --listen 127.0.0.1:3001 --sqlite @migration.sql