use anyhow::Result;
use wasmtime::{component::Component, Store};
use wasmtime_wasi::{ResourceTable, WasiCtxBuilder};

use host::{create_engine, setup_linker, App, HostState, MyHostFunctions};

fn main() -> Result<()> {
    println!("Host: Initializing engine and linker...");
    let engine = create_engine()?;
    let linker = setup_linker(&engine)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let mut store = Store::new(
        &engine,
        HostState {
            table: ResourceTable::new(),
            wasi_ctx,
            host_functions: MyHostFunctions::default(),
        },
    );

    println!("Host: Loading guest component...");
    let component_path = "../guest/target/wasm32-wasip2/debug/guest.wasm";
    let component = Component::from_file(&engine, component_path)?;

    let (app, _instance) = App::instantiate(&mut store, &component, &linker)?;

    let input_string = "Hello from the host!";
    println!(
        "Host: Calling guest's 'run' function with: '{}'",
        input_string
    );

    let result = app.call_run(&mut store, input_string)?;

    println!("Host: Guest returned:");
    println!("'{}'", result);

    Ok(())
}
