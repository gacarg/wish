
#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
mkdir -p neardev
cp target/wasm32-unknown-unknown/release/wish.wasm ./out/main.wasm
near dev-deploy ./out/main.wasm
