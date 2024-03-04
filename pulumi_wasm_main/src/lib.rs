use crate::bindings::exports::component::pulumi_wasm::pulumi_main::Guest;
use bindings::component::pulumi_wasm::pulumi_provider_random_interface::{
    create_random_string, RandomStringArgs,
};
use pulumi_rust_wasm::output::Output;
use crate::bindings::component::pulumi_wasm::pulumi_provider_random_interface;

mod bindings;

struct Component {}

impl Guest for Component {
    fn main() {
        let length: Output<i32> = Output::new(&1234);

        let output_1 = length.get_inner();
        let output_2 = output_1.duplicate();
        let output_2 = unsafe { pulumi_provider_random_interface::Output::from_handle(output_2.into_handle()) };

        let abc = RandomStringArgs {
            name: "test1234".to_string(),
            length: Err(&output_2),
        };
        create_random_string(abc);
    }
}
