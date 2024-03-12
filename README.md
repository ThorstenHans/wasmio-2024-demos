# Wasm I/O 2024 - Demos

This repository contains all samples shown as part of my talk about the WebAssembly Component Model held during Wasm I/O 2024 in Barcelona, Spain.

Both of the following samples have dedicated READMEs to outline how one could build & run them.

## Extensibility

The [extensibility](./extensibility/) demo illustrates how one could use the WebAssembly Component Model to add extensibility to their own application. By relying on WIT contract, extension developers can use any language - that could be compiled into WebAssembly and packaged as a Wasm Component - and use the canonical ABI provided by the component model.

## Composition

The [composition](./composition-sample/) demo shows how WebAssembly components can be used to align with the DRY principle. As part of the demo, a WebAssembly Component (HMAC implementation) is being used in a webhook producer (`acme-product`) which is written in Rust. The same component is used in the webhook consumer (`wonka-3rd-party-service`) which is written in Python.

