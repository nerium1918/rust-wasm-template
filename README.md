# Rust wasm template

This repo aims to be a very simple, yet complete, example of building a wasm binary using a Rust project as source, and then running that wasm within another Rust project.

## How does it work

The example is a Cargo workspace comprised of 2 parts - a `compiler` and a `runner`. The `compiler` is the crate that is responsible for compiling Rust code to a wasm binary using [wasm-pack](https://rustwasm.github.io/wasm-pack/). The `runner` takes that wasm binary, loads it into a Rust process, and executes functions defined in the binary. The `runner` uses [wasmer](https://wasmer.io/).

## How to run it

### Prerequisites

You need to have the following installed:
- Install [Rust](https://www.rust-lang.org/tools/install) (duh)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Add the wasm compilation target - `rustup target add wasm32-unknown-unknown`

Then, follow these steps:
- `cd` inside `compiler` and run `wasm-pack build`
- `cd` back into the workspace root folder and run `cargo run`

If all goes well, you should see `The answer is 42` printed to the console. 
Feel free to open an Issue if you're having trouble running this template.
