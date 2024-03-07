use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use anyhow::Error;
use log::info;
use pulumi_rust_wasm::HASHMAP;
use crate::bindings::exports::component::pulumi_wasm::pulumi_main::Guest;

use pulumi_rust_wasm::output::Output;
use crate::bindings::component::pulumi_wasm::function_reverse_callback::{FunctionInvocationRequest, FunctionInvocationResult, get_functions, set_functions};
use crate::bindings::component::pulumi_wasm::output_interface::{combine_outputs, describe_outputs, non_done_exists};
use crate::random::*;

mod bindings;
bindings::export!(Component with_types_in bindings);

mod random;

struct Component {}

impl Guest for Component {
    fn main() {
        wasm_common::setup_logger();

        let length: Output<i32> = Output::new(&4).map(|i: i32| i * 2);

        let v = create_random_string(RandomStringArgs {
            name: "test123",
            length,
        });

        let new_length = v.map(|s| s.len() as i32);

        let v = create_random_string(RandomStringArgs {
            name: "test123_2",
            length: new_length,
        });

        run_loop().unwrap();

        info!("{}", describe_outputs());

        if non_done_exists() {
            eprintln!("Non done exists");
            panic!("Non done exists");
        }

        // describe_outputs()
    }
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

    let mut functions_map = HASHMAP.lock().unwrap();

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
                    id,
                    value: f(value.to_vec()),
                }
            },
        )
        .collect();

    set_functions(&*mapped);

    Ok(true)
}

