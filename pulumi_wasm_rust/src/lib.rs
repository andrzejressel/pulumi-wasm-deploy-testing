use std::collections::HashMap;
use std::sync::Mutex;
use anyhow::{Context, Error};
use lazy_static::lazy_static;
use log::{error, info};

pub use pulumi_wasm_rust_macro::pulumi_main;
use crate::bindings::component::pulumi_wasm::function_reverse_callback::{FunctionInvocationRequest, FunctionInvocationResult, get_functions, set_functions};
use crate::bindings::component::pulumi_wasm::output_interface::combine_outputs;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "pulumi-wasm-rust",
        path: "../wits/world.wit"
    });
}

pub fn run<F>(f: F) -> Result<(), Error> where F: Fn() -> Result<(), Error> {
    let outer = || {
        wasm_common::setup_logger();
        f()?;
        run_loop()?;
        Ok(())
    };

    let result = outer();

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error running pulumi wasm: [{e}]");
            Err(e)
        }
    }
}

pub fn init() {
    wasm_common::setup_logger();
}

pub fn close() -> Result<(), Error> {
    run_loop()
}

fn run_loop() -> Result<(), Error> {
    loop {
        if !run_all_function()? && !combine_outputs() {
            return Ok(());
        }
    }
}

fn run_all_function(
    // store: &mut Store<SimplePluginCtx>,
    // plugin: &PulumiWasm,
) -> Result<bool, Error> {
    let functions = get_functions("source");

    if functions.is_empty() {
        info!("Functions are empty");
        return Ok(false)
    }

    info!("Functions are not empty");

    let functions_map = HASHMAP.lock().unwrap();

    let mapped: Result<Vec<_>, _> = functions
        .iter()
        .map(
            |FunctionInvocationRequest {
                 id,
                 function_id,
                 value,
             }| {
                info!("Invoking function [{function_id}] with value [{value:?}]");
                let v = rmpv::decode::read_value(&mut value.clone().as_slice())?;
                info!("Invoking function [{function_id}] with value [{v:?}]");
                let f = functions_map.get(function_id).context(format!("Function with id {function_id} not found"))?;
                Ok(FunctionInvocationResult {
                    id,
                    value: f(value.to_vec())?,
                })
            },
        )
        .collect();

    // mapped

    let mapped = match mapped {
        Ok(mapped) => mapped,
        Err(e) => {
            error!("Failed to invoke functions due to [{e}]");
            return Err(e);
        }
    };

    info!("Setting functions");
    set_functions(&mapped);
    info!("run_all_function completed");

    Ok(true)
}


pub mod output;

type Function = Box<dyn Fn(Vec<u8>) -> Result<Vec<u8>, Error> + Send>;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<String, Function>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
