use anyhow::Result;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "../wit",
    world: "app",
});

struct MyHostFunctions;

struct HostState {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    host_functions: MyHostFunctions,
}

impl WasiView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl my::capitalize::host_functions::Host for MyHostFunctions {
    fn capitalize(&mut self, input: String) -> String {
        println!("Host: Received call to 'capitalize' with: '{}'", input);
        input.to_uppercase()
    }
}

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    my::capitalize::host_functions::add_to_linker(&mut linker, |state: &mut HostState| {
        &mut state.host_functions
    })?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let mut store = Store::new(
        &engine,
        HostState {
            table: ResourceTable::new(),
            wasi_ctx,
            host_functions: MyHostFunctions,
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
