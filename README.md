# rust-reqwest-warp-example

This repo is an example of combining [warp](https://github.com/seanmonstar/warp) server library with [reqwest](https://github.com/seanmonstar/reqwest) HTTP client using async-await syntax.
Basic logging setup included.

## Running

With logging output for maximum comfort:

```bash
RUST_LOG=info cargo run
```

To hack on it([cargo-watch](https://github.com/passcod/cargo-watch) recommended):

```bash
cargo watch -x run
```