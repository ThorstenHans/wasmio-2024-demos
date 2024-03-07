#! /bin/bash

pushd acme-product
echo "Building component in" $(pwd)
cargo component build --release > /dev/null
popd

pushd cyberdyne-3rd-party-service
echo "Buidling compontent in" $(pwd)
cargo component build --release > /dev/null
popd

pushd wonka-3rd-party-service
echo "Building consumer in" $(pwd)
source ./.venv/bin/activate
pip install -r requirements.txt > /dev/null
componentize-py -d wit -w consumer componentize -m spin_sdk=spin-http app -o app.wasm > /dev/null
deactivate
popd

pushd hmac
echo "Building component in" $(pwd)
cargo component build --release > /dev/null
popd

echo "Composing ACME Product..."
wasm-tools compose -d ./hmac/target/wasm32-wasi/release/hmac.wasm ./acme-product/target/wasm32-wasi/release/api.wasm -o ./acme-product/composed.wasm

echo "Composing Cyberdyne Service..."
wasm-tools compose -d ./hmac/target/wasm32-wasi/release/hmac.wasm ./cyberdyne-3rd-party-service/target/wasm32-wasi/release/consumer.wasm -o ./cyberdyne-3rd-party-service/composed.wasm

echo "Composing Wonka Service..."
 wasm-tools compose -d ./hmac/target/wasm32-wasi/release/hmac.wasm ./wonka-3rd-party-service/app.wasm -o ./wonka-3rd-party-service/composed.wasm 
