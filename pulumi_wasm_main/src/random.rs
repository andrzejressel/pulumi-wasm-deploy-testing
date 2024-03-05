use pulumi_rust_wasm::output::Output;
use crate::bindings::component::pulumi_wasm::{output_interface, pulumi_provider_random_interface};

pub struct RandomStringArgs {
    pub(crate) name: String,
    pub(crate) length: Output<i32>,
}

pub fn create_random_string(
    args: RandomStringArgs,
) {
    let length = clone(args.length);
    let args = pulumi_provider_random_interface::RandomStringArgs {
        name: args.name,
        length: &length,
    };
    pulumi_provider_random_interface::create_random_string(args);
}

fn clone<T>(output: Output<T>) -> output_interface::Output {
    unsafe {
        let inner = output.get_inner();
        let cloned = inner.duplicate();
        pulumi_provider_random_interface::Output::from_handle(cloned.into_handle())
    }
}