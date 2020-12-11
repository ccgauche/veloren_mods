@echo off
echo Builiding WASM Plugin
cd ./wasmplugin
cargo build --release --target wasm32-unknown-unknown
echo Builiding Host
cd ..
cargo run --release