.PHONY build-app-plugins: ensure-plugin-dir build-uppercase build-obfuscate build-reverse

.PHONY ensure-plugin-dir:
ensure-plugin-dir:
	mkdir -p active
	pushd active; \
	rm *.wasm
.PHONY build-uppercase:
build-uppercase: 
	pushd plugins; \
	pushd uppercase; \
	cargo build --target wasm32-unknown-unknown --release
	wasm-tools component new ./plugins/uppercase/target/wasm32-unknown-unknown/release/uppercase.wasm -o ./active/uppercase.component.wasm

.PHONY build-obfuscate:
build-obfuscate: 
	pushd plugins; \
	pushd obfuscate; \
	cargo build --target wasm32-unknown-unknown --release
	wasm-tools component new ./plugins/obfuscate/target/wasm32-unknown-unknown/release/obfuscate.wasm -o ./active/obfuscate.component.wasm

.PHONY build-reverse:
build-reverse:
	pushd plugins; \
	pushd reverse; \
	jco componentize reverse.js --wit ./wit/wasmio.wit -o reverse.wasm
	wasm-tools strip ./plugins/reverse/reverse.wasm -o ./active/reverse.component.wasm

.PHONY run:
run:
	pushd transformer; \
	cargo run --release