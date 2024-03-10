use anyhow::{Context, Error};
use log::{error, info};
use pulumi_wasm_provider_random_rust::random::{create_random_string, RandomStringArgs};
use pulumi_wasm_rust::{HASHMAP, pulumi_main};

use pulumi_wasm_rust::output::Output;

// mod bindings;

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    wasm_common::setup_logger();

    let length: Output<i32> = Output::new(&12).map(|i: i32| i * 3);

    let _ = create_random_string(RandomStringArgs {
        name: "test",
        length,
    });

    Ok(())
}
