use wasmer::{Store, Module, Instance, imports};
use anyhow::Result;

fn main() -> Result<()> {
    let mut store = Store::default();
    let module = Module::from_file(&store, "/Users/petko/misc/wasm-rust/compiler/pkg/library_bg.wasm")?;

    let import_object = imports! {
        "fd_write" => {}
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let basic = instance.exports.get_function("basic")?;
    let result = basic.call(&mut store, &[])?[0].to_string();
    println!("The answer is {}", result);

    Ok(())
}
