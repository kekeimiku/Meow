cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release --target wasm32-unknown-unknown

wasm-opt -Oz -o r1.wasm r1.wasm