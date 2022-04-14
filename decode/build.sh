cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release --target wasm32-unknown-unknown

wasm-opt -Oz -o r0.wasm r0.wasm