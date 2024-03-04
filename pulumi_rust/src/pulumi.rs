use crate::grpc::resource_monitor_client::ResourceMonitorClient;
use crate::grpc::RegisterResourceRequest;
use crate::pulumi::server::Main;
use anyhow::Error;
use prost::Message;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use tonic::async_trait;
use uuid::Uuid;
use wasmtime::component::{Component, Instance, Linker, ResourceAny, ResourceTable};
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
        use futures::executor;
        let result = self.register_async(request).await?;
        // executor::block_on(result)?;
        Ok(vec![])
        // run_blocking(self.register_async(request))
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

        // Plugin::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| &mut state.logger).unwrap();

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
        self
            .plugin
            .component_pulumi_wasm_pulumi_main()
            .call_main(store).await?;

        Ok(())


        // self.plugin
        //     .component_pulumi_wasm_pulumi_interface()


    }

    // pub async fn test(&self) -> Result<(), Error> {
    //     let object = prost_types::Struct {
    //         fields: BTreeMap::from([
    //             (
    //                 "length".to_string(),
    //                 prost_types::Value {
    //                     kind: Option::from(prost_types::value::Kind::NumberValue(10.into())),
    //                 },
    //             ),
    //             (
    //                 "minNumeric".to_string(),
    //                 prost_types::Value {
    //                     kind: Option::from(prost_types::value::Kind::NumberValue(5.into())),
    //                 },
    //             ),
    //         ]),
    //     };
    //
    //     let request = RegisterResourceRequest {
    //         r#type: "random:index/randomString:RandomString".to_string(),
    //         name: "test_string_123".to_string(),
    //         parent: "".to_string(),
    //         custom: true,
    //         object: Some(object),
    //         protect: false,
    //         dependencies: vec![],
    //         provider: "".to_string(),
    //         property_dependencies: Default::default(),
    //         delete_before_replace: false,
    //         version: "".to_string(),
    //         ignore_changes: vec![],
    //         accept_secrets: true,
    //         additional_secret_outputs: vec![],
    //         alias_ur_ns: vec![],
    //         import_id: "".to_string(),
    //         custom_timeouts: None,
    //         // custom_timeouts: Some(CustomTimeouts {
    //         //     create: "5m".to_string(),
    //         //     update: "5m".to_string(),
    //         //     delete: "5m".to_string(),
    //         // }),
    //         delete_before_replace_defined: false,
    //         supports_partial_values: false,
    //         remote: false,
    //         accept_resources: false,
    //         providers: Default::default(),
    //         replace_on_changes: vec![],
    //         plugin_download_url: "".to_string(),
    //         plugin_checksums: Default::default(),
    //         retain_on_delete: false,
    //         aliases: vec![],
    //         deleted_with: "".to_string(),
    //         alias_specs: false,
    //         source_position: None,
    //     };
    //
    //     let request_vec = request.encode_to_vec();
    //
    //     self.register_resource(request_vec).await?;
    //
    //     Ok(())
    // }
    //
    // pub async fn register_resource(&self, content: Vec<u8>) -> Result<(), Error> {
    //     use prost::Message;
    //     let engine_url = self
    //         .pulumi_monitor_url
    //         .clone()
    //         .ok_or(Error::msg("engine_url not set"))?;
    //
    //     let request = RegisterResourceRequest::decode(&mut content.clone().as_slice())?;
    //
    //     let mut client = ResourceMonitorClient::connect(format!("tcp://{engine_url}")).await?;
    //
    //     let result = client.register_resource(request).await?;
    //     use std::io::prelude::*;
    //
    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .append(true)
    //         .open("my-file")
    //         .unwrap();
    //
    //     writeln!(file, "{:?}", result).unwrap();
    //
    //     Ok(())
    // }
}
