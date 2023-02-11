This repo aims to be a very simple, yet complete, example of building a WASM binary using a Rust project as source, and then running that WASM within another Rust project.

The example is a Cargo workspace comprised of 2 parts - a `compiler` and a `runner`. The `compiler` is the crate that is responsible for compiling Rust code to a WASM binary using [wasm-pack](https://rustwasm.github.io/wasm-pack/). The `runner` takes that WASM binary, loads it into a Rust process, and executes functions defined in the binary. The `runner` uses [wasmer](https://wasmer.io/).
