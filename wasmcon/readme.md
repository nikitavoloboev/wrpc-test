# Rust Wasm components with wRPC

Example from [wRPC: Distributed Components, No Assembly Required](https://youtu.be/EYqZYXjCvkY?si=BlgYtbGD1DcJKUPf&t=1382) talk.

## Build

With `cargo`:

```
cargo build --workspace --target wasm32-wasip2 --release
```

With `nix`:

```
nix build .#wasmcon-wasm32-wasip2
```

## Run

### Server

```
wrpc-wasmtime tcp serve ./target/wasm32-wasip2/release/server.wasm
```

### Client

```
wrpc-wasmtime tcp run ./target/wasm32-wasip2/release/client.wasm
```
