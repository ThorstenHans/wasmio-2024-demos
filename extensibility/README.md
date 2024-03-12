# Extensibility using the WebAssembly Component Model

This demo illustrates how one could roll their own extensibility system using the WebAssembly Component Model. Although the sample itself is fairly simple, it shows how one could dynamically load WebAssembly Components exporting a desired functionality.

## Prerequisites

This sample combines artifacts built in different programming languages. That said, ensure the following is installed on your system:

 - Rust `1.75.0` with `wasm32-wasi` target
   - `cargo-component` (https://github.com/bytecodealliance/cargo-component) 
 - Node.js `21.6.2`
   - `jco` - (https://github.com/bytecodealliance/jco)
   - `componentize-js` (https://github.com/bytecodealliance/ComponentizeJS)

## The Host Application

The `transformer` application is a simple CLI application, which asks users to input text values. It takes the user input and chains it through all plugins (WebAssembly Components) exporting a `transform` function. 

The host application is written in Rust & build for your local architecture (e.g., `x64` or `arm64`). It has a dependency on the `wasmtime` crate (with the `component-model` feature). 

See the [WIT world of the host](./transformer/wit/wasmio.wit) defining functionality that each plugin must provide. 

## The Plugins

There are multiple plugins located in the `./plugins` sub-directory. 

### The `uppercase` Plugin

This plugin is written in Rust. It has been created and is built using `cargo component` (See prerequisites).

### The `reverse` Plugin

This plugin is written in JavaScript. It requires `jco` and `componentize.js` (See prerequisites) for creating a WebAssembly Component from the source code.

### The `obfuscate` Plugin

This plugin is written in Rust. It has been created and is built using `cargo component` (See prerequisites).

## Runtime Behavior

Plugins are only loaded from the `./active` sub-directory. While demonstrating, one could simple add or remove WebAssembly Components (exporting the `transform`) functionality to the `./active` folder. The host application will load plugins automatically everytime the user provides a new value.

## Running the Application

You can run the host application usign `make run`.

In a different terminal instance, you can compile all plugins using the `make build-all-plugins`. Resulting WebAssembly Components will be placed in the root folder of this sample.

To activate the `uppercase` plugin, simply copy it to the `./active` folder: `cp uppercase.wasm ./active`

There are make targets to activate all plugins (`make activate-all-plugins`) and to deactivate all plugins again (`make deactivate-all-plugins`).