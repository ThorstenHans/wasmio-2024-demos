.PHONY build:
build:
	cargo component build --release
	wasm-tools compose -d ./../hmac/target/wasm32-wasi/release/hmac.wasm ./target/wasm32-wasi/release/acme.wasm -o ./composed.wasm