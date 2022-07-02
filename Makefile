.PHONY: node wasm

WASM_BINDGEN=wasm-bindgen

node: node/decancer.node
wasm: wasm/pkg/decancer_wasm_bg.wasm	

node/decancer.node: node/src/lib.js
	@cd node
	node ./node_modules/@napi-rs/cli/scripts/index.js build --release
	@cd ..

node/src/lib.js: node/src/node_modules/.package-lock.json
	@cd node
	npx tsc --project tsconfig.json
	@cd ..

node/src/node_modules/.package-lock.json:
	@cd node
	npm install --save-dev
	@cd ..

wasm/pkg/decancer_wasm_bg.wasm:
	@cd wasm
	cargo install wasm-bindgen-cli
	$(WASM_BINDGEN) build --no-typescript --release -t web
	@cd ..
