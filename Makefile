clean:
	rm -rf target/

build:
	cargo build

build-prod:
	cargo build --release

build-lib:
	cargo build --lib

build-lib-prod:
	cargo build --lib --release

build-wasm:
	cargo build --lib --target wasm32-unknown-unknown

build-wasm-prod:
	cargo build --lib --release --target wasm32-unknown-unknown

copy:
	cp target/wasm32-unknown-unknown/release/hello_rust.wasm www/assets/wasm/

serve:
	cd www && python3 -m http.server 8080
