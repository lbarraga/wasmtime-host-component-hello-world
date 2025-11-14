use anyhow::Result;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
// FIX 1: Import from the root of `wasmtime_wasi` (Preview 1)
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

// 1. Generate the Wasmtime bindings for our "app" world
wasmtime::component::bindgen!({
    path: "../wit",
    world: "app",
});

// 2. Define the struct to hold our host function implementations
struct MyHostFunctions;

// 3. Define the HostState, which holds WASI state and our functions
struct HostState {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    host_functions: MyHostFunctions,
}

// 4. Implement WasiView (Preview 1) for our HostState
// FIX 2: This is the Preview 1 trait, matching your sample code
impl WasiView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

// 5. Implement the actual host functions
// (This is unchanged)
impl my::capitalize::host_functions::Host for MyHostFunctions {
    fn capitalize(&mut self, input: String) -> String {
        println!("Host: Received call to 'capitalize' with: '{}'", input);
        // Simply convert the input string to uppercase
        input.to_uppercase()
    }
}

// 6. The main application logic
fn main() -> Result<()> {
    // --- Standard Wasmtime Setup ---
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    // --- Create a Linker ---
    let mut linker = Linker::new(&engine);

    // FIX 3: Link standard WASI (Preview 1)
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    // Link our custom host functions
    my::capitalize::host_functions::add_to_linker(
        &mut linker,
        |state: &mut HostState| &mut state.host_functions,
    )?;

    // --- Set up the Store ---
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let mut store = Store::new(
        &engine,
        HostState {
            table: ResourceTable::new(),
            wasi_ctx,
            host_functions: MyHostFunctions, // Initialize our functions
        },
    );

    // --- Load and Instantiate the Component ---
    println!("Host: Loading guest component...");
    let component_path = "../guest/target/wasm32-wasip2/debug/guest.wasm";

    let component = Component::from_file(&engine, component_path)?;

    // Instantiate the component (synchronous)
    let (app, _instance) = App::instantiate(&mut store, &component, &linker)?;

    // --- Run the Component ---
    let input_string = "Hello from the host!";
    println!("Host: Calling guest's 'run' function with: '{}'", input_string);

    // Call the guest's exported 'run' function (synchronous)
    let result = app.call_run(&mut store, input_string)?;

    // Print the final result from the guest
    println!("Host: Guest returned:");
    println!("'{}'", result);

    Ok(())
}