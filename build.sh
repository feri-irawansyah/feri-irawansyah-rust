wasm-pack build --target=web -- --features hydrate --no-default-features

cp -f css/* pkg/
cp -f static/* pkg/

worker-build --release -- --features "ssr console_error_panic_hook" --no-default-features --bin example