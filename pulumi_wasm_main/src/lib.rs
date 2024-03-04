use crate::bindings::exports::component::pulumi_wasm::pulumi_main::Guest;
use bindings::component::pulumi_wasm::pulumi_provider_random_interface::{
    create_random_string, RandomStringArgs,
};
use pulumi_rust_wasm::output::Output;

mod bindings;

struct Component {}

impl Guest for Component {
    fn main() {
        let _output: Output<i32> = Output::new(&123);

        let abc = RandomStringArgs {
            name: "test123".to_string(),
            length: Ok(1),
        };
        create_random_string(abc);
    }
}
