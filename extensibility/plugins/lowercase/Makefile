.PHONY: build
build:
	source ./.venv/bin/activate; \
	pip install -r requirements.txt; \
	componentize-py -d wit -w wasmio:samples/plugin componentize lowercase -o lowercase.wasm; \
	deactivate