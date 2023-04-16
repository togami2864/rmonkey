dev:
	cd playground && pnpm run dev

build-web:
	make build-wasm
	cd playground && pnpm run build

build-wasm:
	cd crates/rmonkey_wasm && wasm-pack build

build:
	cargo build