use crate::grpc::resource_monitor_client::ResourceMonitorClient;
use crate::grpc::RegisterResourceRequest;
use crate::pulumi::server::Main;
use anyhow::Error;
use prost::Message;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::ops::DerefMut;
use std::rc::Rc;
use tonic::async_trait;
use wasmtime::component::{Component, Instance, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct Pulumi {
    plugin: Main,
    _instance: Instance,
    store: RefCell<Store<SimplePluginCtx>>,
    pulumi_monitor_url: Option<String>,
}

pub(crate) mod server {
    wasmtime::component::bindgen!({
        path: "../wits/world.wit",
        world: "main",
        async: true
    });
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
    my_state: MyState,
}

struct MyState {
    functions: HashMap<String, Box<dyn Fn(Vec<u8>) -> Vec<u8> + Send>>,
    pulumi_monitor_url: Option<String>,
}

#[async_trait]
impl server::component::pulumi_wasm::external_world::Host for MyState {
    async fn register_resource(&mut self, request: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
        let _result = self.register_async(request).await?;
        Ok(vec![])
    }
}

impl MyState {
    async fn register_async(&mut self, request: Vec<u8>) -> wasmtime::Result<()> {
        use prost::Message;
        let engine_url = self
            .pulumi_monitor_url
            .clone()
            .ok_or(Error::msg("engine_url not set"))?;

        let request = RegisterResourceRequest::decode(&mut request.as_slice())?;

        let mut client = ResourceMonitorClient::connect(format!("tcp://{engine_url}")).await?;

        let result = client.register_resource(request).await?;
        use std::io::prelude::*;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("my-file")
            .unwrap();

        writeln!(file, "{:?}", result).unwrap();

        Ok(())
    }
}

impl WasiView for SimplePluginCtx {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}

impl Pulumi {
    pub async fn create(
        pulumi_wasm_file: &str,
        pulumi_monitor_url: &Option<String>,
    ) -> Result<Rc<Pulumi>, Error> {
        let mut engine_config = wasmtime::Config::new();
        engine_config.wasm_component_model(true);
        engine_config.async_support(true);
        engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        engine_config.debug_info(true);

        let engine = wasmtime::Engine::new(&engine_config).unwrap();

        let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
        Main::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
            &mut state.my_state
        })?;

        wasmtime_wasi::preview2::command::add_to_linker(&mut linker).unwrap();

        let table = ResourceTable::new();

        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build();

        let mut store = Store::new(
            &engine,
            SimplePluginCtx {
                // logger: SimpleLogger {},
                table,
                context: wasi_ctx,
                my_state: MyState {
                    functions: HashMap::new(),
                    pulumi_monitor_url: pulumi_monitor_url.clone(),
                },
            },
        );

        let component = Component::from_file(&engine, pulumi_wasm_file)?;

        let (plugin, instance) = Main::instantiate_async(&mut store, &component, &linker).await?;

        let store = RefCell::new(store);

        Ok(Rc::new(Pulumi {
            plugin,
            _instance: instance,
            store,
            // pulumi_monitor_url: None,
            pulumi_monitor_url: pulumi_monitor_url.clone(),
        }))
    }

    pub async fn start(&self) -> Result<(), Error> {
        let mut binding = self.store.borrow_mut();
        let store = binding.deref_mut();
        self.plugin
            .component_pulumi_wasm_pulumi_main()
            .call_main(store)
            .await?;

        Ok(())
    }
}
