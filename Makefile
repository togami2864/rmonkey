all:
	pnpm install --frozen-lockfile
	cargo build
	make build-wasm
	make build-web
dev:
	make build-wasm
	cd playground && pnpm install && pnpm dev

build-web:
	make build-wasm
	pnpm build

build-wasm:
	cd crates/rmonkey_wasm && wasm-pack build

build:
	cargo build