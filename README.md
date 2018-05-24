# rust-wasm-hello-world

```
rustup update
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/hello_world.wasm -o hello_world.gc.wasm
```

- [How to pass a string from JavaScript to Rust in WebAssembly](https://stackoverflow.com/a/49020435/412417)
- [How to return a string from Rust in WebAssembly](https://stackoverflow.com/a/47676844/412417)
- [Static Mut Variable in Rust](https://stackoverflow.com/a/19608953/412417)
