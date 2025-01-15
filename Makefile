run:
	cargo run

build:
	cargo build

build-release:
	cargo build --release

install-wasm-prereqs:
	cargo install -f wasm-bindgen-cli --version 0.2.99
	cargo install wasm-server-runner

install-wasm: install-wasm-prereqs
	rustup target install wasm32-unknown-unknown

run-wasm: install-wasm
	cargo run --release --target wasm32-unknown-unknown

watch-wasm:
	cargo watch -cx "run --release --target wasm32-unknown-unknown"

build-wasm: install-wasm
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/fortress.wasm