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
use wasmtime::component::{Component, Instance, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};
use std::io::prelude::*;
use async_trait::async_trait;
use log::Log;
use regex::Regex;

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
    async fn is_in_preview(&mut self) -> wasmtime::Result<bool> {
        Ok(std::env::var("PULUMI_DRY_RUN").is_ok())
    }
    async fn register_resource(&mut self, request: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
        Ok(self.register_async(request).await?)
    }
}

#[async_trait]
impl crate::pulumi::server::component::pulumi_wasm::log::Host for MyState {
    async fn log(&mut self, content: crate::pulumi::server::component::pulumi_wasm::log::Content) -> wasmtime::Result<()> {
        log::logger().log(&log::Record::builder()
            .metadata(log::Metadata::builder()
                .level(match content.level {
                    crate::pulumi::server::component::pulumi_wasm::log::Level::Trace => log::Level::Trace,
                    crate::pulumi::server::component::pulumi_wasm::log::Level::Debug => log::Level::Debug,
                    crate::pulumi::server::component::pulumi_wasm::log::Level::Info => log::Level::Info,
                    crate::pulumi::server::component::pulumi_wasm::log::Level::Error => log::Level::Error,
                    crate::pulumi::server::component::pulumi_wasm::log::Level::Warn => log::Level::Warn,
                })
                .target(&content.target)
                .build()
            )
            .args(format_args!("{}", content.args))
            .module_path(content.module_path.as_deref())
            .file(content.file.as_deref())
            .line(content.line)
            .key_values(&content.key_values.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<(&str, &str)>>())
            .build());

        Ok(())
    }
}

impl MyState {
    async fn register_async(&mut self, request: Vec<u8>) -> wasmtime::Result<Vec<u8>> {
        use prost::Message;
        let engine_url = self
            .pulumi_monitor_url
            .clone()
            .ok_or(Error::msg("engine_url not set"))?;

        let request = RegisterResourceRequest::decode(&mut request.as_slice())?;

        let mut client = ResourceMonitorClient::connect(format!("tcp://{engine_url}")).await?;

        let result = client.register_resource(request).await?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("my-file")
            .unwrap();

        writeln!(file, "{:?}", result).unwrap();

        Ok(result.get_ref().encode_to_vec())
    }
}

impl WasiView for SimplePluginCtx {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
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
