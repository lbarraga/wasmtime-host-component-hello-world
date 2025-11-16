use anyhow::Result;
use wasmtime::{component::Linker, Config, Engine};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

pub mod host_impl;

wasmtime::component::bindgen!({
    path: "../wit",
    world: "app",
});

use my::capitalize::host_functions;

pub use host_impl::MyHostFunctions;

pub struct HostState {
    pub table: ResourceTable,
    pub wasi_ctx: WasiCtx,
    pub host_functions: MyHostFunctions,
}

impl WasiView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

pub fn create_engine() -> Result<Engine> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    Engine::new(&config)
}

pub fn setup_linker(engine: &Engine) -> Result<Linker<HostState>> {
    let mut linker = Linker::new(engine);

    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    host_functions::add_to_linker(&mut linker, |state: &mut HostState| {
        &mut state.host_functions
    })?;

    Ok(linker)
}
