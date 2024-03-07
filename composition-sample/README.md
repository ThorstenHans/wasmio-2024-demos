# Composition Example

This sample demonstrates how to use WebAssembly Components within a Spin application.

## Apps

- [ACME](./acme/) - acts as an webhook producer (localhost:3000)
- [CYBERDYNE](./cyberdyne/) - acts as a webhook consumer (localhost:3001)
- [WONKA](./wonka/) - acts as a webhook consumer (localhost:3002)

## Middlewares

- [hmac](./hmac/) - provides capabilities to `sign` data and `verify` those signatures

## Prerequisites

- [Rust](https://rust-lang.org) with `wasm32-wasi` target must be installed
- [Cargo Component](https://github.com/bytecodealliance/cargo-component) must be installed
- [Wasm Tools](https://github.com/bytecodealliance/wasm-tools) must be installed
- [Spin](https://developer.fermyon.com/spin) must be installed

## Building the sample

Use the [Makefile](./Makefile) to compile and compose all necessary Wasm Components.

## Running the sample

Running each of the following in a dedicated terminal session:
  - `make start-acme`
  - `make start-wonka`
  - `make start-cyberdyne`

## Using the sample

Run `make register-wonka` (as of writing this, this removes all previously registered webhooks for demonstration purposes) to register the wonka service as webhook consumer.
Run `make fire-webhook` to make ACME fire a webhook