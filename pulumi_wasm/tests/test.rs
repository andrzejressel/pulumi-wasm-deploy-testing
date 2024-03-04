use crate::server::exports::component::pulumi_wasm::function_reverse_callback::FunctionInvocationRequest;
use crate::server::PulumiWasm;
use anyhow::{Error, Ok};
use serde::{Deserialize};
use std::collections::HashMap;
use std::string::String;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::Store;
use wasmtime_wasi::preview2::WasiCtx;
use wasmtime_wasi::preview2::WasiCtxBuilder;
use wasmtime_wasi::preview2::WasiView;

mod server {
    wasmtime::component::bindgen!({
        path: "../wits/world.wit",
        world: "pulumi-wasm"
    });
}

struct SimplePluginCtx {
    table: ResourceTable,
    context: WasiCtx,
    my_state: MyState,
}

fn create_function<A, B, F>(f: F) -> impl Fn(Vec<u8>) -> Vec<u8> + Send
where
    F: Fn(A) -> B + Send,
    A: serde::de::DeserializeOwned,
    B: serde::Serialize,
{
    move |arg: Vec<u8>| {
        let arg = arg.clone();
        let argument = rmp_serde::decode::from_slice(&arg).unwrap();
        let result = f(argument);
        rmp_serde::to_vec(&result).unwrap()
    }
}

struct MyState {
    functions: HashMap<String, Box<dyn Fn(Vec<u8>) -> Vec<u8> + Send>>,
}

impl server::component::pulumi_wasm::external_world::Host for MyState {
    fn register_resource(&mut self, _request: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        todo!()
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

#[test]
fn should_return_output_value() -> Result<(), Error> {
    let (mut store, plugin) = create_engine()?;

    let output1 = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_constructor(&mut store, rmp_serde::to_vec("123").unwrap().as_slice())?;

    let output1_value = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_get(&mut store, output1)?;

    anyhow::ensure!(output1_value.unwrap() == rmp_serde::to_vec("123").unwrap());

    Ok(())
}

#[test]
fn should_not_return_value_of_unhandled_mapped_value() -> Result<(), Error> {
    let (mut store, plugin) = create_engine()?;

    let function_name = "add_world_suffix";

    store.data_mut().my_state.functions.insert(
        function_name.to_string(),
        Box::new(create_function(|arg: String| format!("{arg} World"))),
    );

    let output1 = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_constructor(&mut store, rmp_serde::to_vec("Hello").unwrap().as_slice())?;

    let output2 = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_map(&mut store, output1, function_name)?;

    let output2_value = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_get(&mut store, output2)?;

    anyhow::ensure!(output2_value == None);

    Ok(())
}

#[test]
fn should_return_value_of_handled_mapped_value() -> Result<(), Error> {
    let (mut store, plugin) = create_engine()?;

    let function_name = "add_world_suffix";

    store.data_mut().my_state.functions.insert(
        function_name.to_string(),
        Box::new(create_function(|arg: String| format!("{arg} World"))),
    );

    let output1 = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_constructor(&mut store, rmp_serde::to_vec("Hello").unwrap().as_slice())?;

    let output2 = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_map(&mut store, output1, function_name)?;

    let _ = run_all_function(&mut store, &plugin)?;

    let output2_value = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_get(&mut store, output2)?;

    anyhow::ensure!(output2_value.unwrap() == rmp_serde::to_vec("Hello World").unwrap());

    Ok(())
}

#[ignore]
#[test]
fn should_create_struct() -> Result<(), Error> {
    let (mut store, plugin) = create_engine()?;

    #[derive(Deserialize, PartialEq)]
    struct S {
        string: String,
        int: i32,
    }

    let string = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_constructor(&mut store, rmp_serde::to_vec("String").unwrap().as_slice())?;

    let u32 = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_constructor(&mut store, rmp_serde::to_vec(&32i32).unwrap().as_slice())?;

    let r#struct = plugin
        .component_pulumi_wasm_output_interface()
        .call_create_struct(
            &mut store,
            vec![("string".to_string(), string), ("int".to_string(), u32)].as_slice(),
        )?;

    let struct_value = plugin
        .component_pulumi_wasm_output_interface()
        .output()
        .call_get(&mut store, r#struct)?
        .unwrap();

    let expected = S {
        string: "String".to_string(),
        int: 32,
    };

    let actual = rmp_serde::decode::from_slice::<S>(struct_value.as_slice())?;

    anyhow::ensure!(expected == actual);

    Ok(())
}

fn create_engine() -> Result<(Store<SimplePluginCtx>, PulumiWasm), Error> {
    let mut engine_config = wasmtime::Config::new();
    engine_config.wasm_component_model(true);
    // engine_config.async_support(true);
    engine_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    engine_config.debug_info(true);

    let engine = wasmtime::Engine::new(&engine_config).unwrap();

    let mut linker: Linker<SimplePluginCtx> = Linker::new(&engine);
    PulumiWasm::add_to_linker(&mut linker, |state: &mut SimplePluginCtx| {
        &mut state.my_state
    })?;

    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker).unwrap();

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
            },
        },
    );

    let component = Component::from_file(&engine, "../target/wasm32-wasi/debug/pulumi_wasm.wasm")?;

    let (plugin, _instance) = PulumiWasm::instantiate(&mut store, &component, &linker)?;
    Ok((store, plugin))
}

#[test]
fn should_return_none_when_output_is_empty() {}

fn run_all_function(
    store: &mut Store<SimplePluginCtx>,
    plugin: &PulumiWasm,
) -> Result<(), anyhow::Error> {
    use crate::server::exports::component::pulumi_wasm::function_reverse_callback::FunctionInvocationResult;
    use std::borrow::BorrowMut;

    let functions = plugin
        .component_pulumi_wasm_function_reverse_callback()
        .call_get_functions(store.borrow_mut(), "source")?;

    let functions_map = &store.data_mut().my_state.functions;

    let mapped: Vec<_> = functions
        .iter()
        .map(
            |FunctionInvocationRequest {
                 id,
                 function_id,
                 value,
             }| {
                let f = functions_map.get(function_id).unwrap();
                FunctionInvocationResult {
                    id: *id,
                    value: f(value.to_vec()),
                }
            },
        )
        .collect();

    plugin
        .component_pulumi_wasm_function_reverse_callback()
        .call_set_functions(store.borrow_mut(), mapped.as_slice())?;

    Ok(())
}
