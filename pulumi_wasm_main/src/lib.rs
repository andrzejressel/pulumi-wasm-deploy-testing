use rmpv::Integer;
use bindings::component::pulumi_wasm::pulumi_provider_random_interface::{create_random_string, RandomStringArgs};
use pulumi_rust_wasm::output::Output;
use crate::bindings::exports::component::pulumi_wasm::pulumi_main::Guest;

mod bindings;

struct Component {}

impl Guest for Component {
    fn main() {
        let output: Output<i32> = Output::new(&123);

        let abc = RandomStringArgs {
            name: "test123".to_string(),
            length: Ok(1),
        };
        create_random_string(abc);
    }
}
