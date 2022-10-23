cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/henry-bbs.wasm --out-dir wasm --no-modules --no-typescript
sudo cp wasm/* /usr/share/nginx/html
