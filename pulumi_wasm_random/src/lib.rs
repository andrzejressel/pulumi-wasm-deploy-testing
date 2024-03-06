use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest};
use crate::bindings::exports::component::pulumi_wasm::pulumi_provider_random_interface::{Guest, RandomStringArgs, RandomStringResult};

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl Guest for Component {
    fn create_random_string(args: RandomStringArgs) -> RandomStringResult {
        let r#type = "random:index/randomString:RandomString".to_string();

        let request = RegisterResourceRequest {
            type_: r#type,
            name: args.name,
            object: vec![ObjectField { name: "length".to_string(), value: &args.length }],
        };

        let o = register(&request);

        RandomStringResult {
            result: o,
        }
    }

    fn handle_functions() {
        todo!()
    }
}
