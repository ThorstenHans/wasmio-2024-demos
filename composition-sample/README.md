# Composition Example

This sample demonstrates how to use WebAssembly Components within a Spin application.

## Apps

- [API](./api/) - a HTTP API that can be used to register for webhooks and fire them
- [Consumer](./consumer/) - a sample webhook consumer

## Middlewares

- [hmac](./hmac/) - provides capabilities to `sign` data and `verify` those signatures

## Prerequisites

- [Rust](https://rust-lang.org) with `wasm32-wasi` target must be installed
- [Cargo Component](https://github.com/bytecodealliance/cargo-component) must be installed
- [Wasm Tools](https://github.com/bytecodealliance/wasm-tools) must be installed
- [Spin](https://developer.fermyon.com/spin) must be installed

## Building the sample

Use the [build.sh](./build.sh) script to compile and compose all necessary Wasm Components.

## Running the sample

- In a dedicated terminal instance invoke `cd api && spin up --sqlite @migration.sql`
- In a dedicated terminal instance invoke `cd consumer && spin up --listen 127.0.0.1:3001 --sqlite @migration.sql`

## Using the sample with `tests.restbook`

You can use [`tests.restbook`](./tests.restbook) for using the sample. To do so, install the `tanhakabir.rest-book` extension first, and follow the instructions mentioned in the notebook.

## Using the sample

Register the webhook consumer as shown here:

```
curl -X POST http://localhost:3000/register \
 -H 'Content-Type:application/json' \
 -d '{ "url": "http://localhost:3001/target", "event": "sample_event"}'
```

- The request above is handled by the [api](./api/), the consumer is stored in the sqlite database.
- [api](./api/) generates unique key data for the consumer
- [api](./api/) invokes `http://localhost:3001/target?handshake=true` via `POST` and sends the key data as payload (`{"keyData": ""}`)
- [consumer](./consumer/) stores key data in its database (dedicated sqlite database)
- [consumer](./consumer/) must respond with `200` otherwise registration fails
- [api](./api) returns `201` if succeeded

Demonstrate webhook firing

```bash
curl -X POST http://localhost:3000/fire
```

- The request above is handled by [api](./api/)
- All webhook registrations are loaded from sqlite
- A sample payload is created
- for every registration
  - payload is singed with the registration specific key data using `hmac` (implemented in [hmac](./hmac/) (dedicated Wasm component))
  - signature is set as HTTP header `X-Signature`
  - outbound http request is sent via `POST` to the url of the registration
  - [consumer](./consumer/) endpoint is invoked
    - the key data is loaded from consumer's database
    - signature is read from incoming HTTP header `X-Signature`
    - data is read from request body
    - `hmac` is verified (implemented in [hmac](./hmac/) (dedicated Wasm component))
    - if signature is **valid** `200` is returned
    - if signature is **invalid** `500` is returned
  - HTTP response code of the consumer is logged to `stdout`
- [api](./api/) returns with `200`
