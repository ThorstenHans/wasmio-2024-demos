.PHONY build-app-plugins: build-uppercase build-obfuscate build-reverse

.PHONY build-uppercase:
build-uppercase: 
	pushd plugins; \
	pushd uppercase; \
	cargo component build --target wasm32-unknown-unknown --release; \
	mv ./target/wasm32-unknown-unknown/release/uppercase.wasm ../../

.PHONY build-obfuscate:
build-obfuscate: 
	pushd plugins; \
	pushd obfuscate; \
	cargo component build --target wasm32-unknown-unknown --release; \
	mv ./target/wasm32-unknown-unknown/release/obfuscate.wasm ../../

.PHONY build-reverse:
build-reverse:
	pushd plugins; \
	pushd reverse; \
	jco componentize reverse.js --wit ./wit/wasmio.wit -o reverse.wasm; \
	wasm-tools strip ./reverse.wasm -o ../../reverse.wasm

.PHONY build-host:
build-host:
	pushd transformer; \
	cargo build --release

.PHONY run:
run:
	pushd transformer; \
	cargo run --release

.PHONY activate-all-plugins:
activate-all-plugins:
	cp *.wasm ./active/
