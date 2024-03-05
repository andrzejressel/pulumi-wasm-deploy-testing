use crate::bindings::component::pulumi_wasm::register_interface::{ObjectField, register, RegisterResourceRequest};
use crate::bindings::exports::component::pulumi_wasm::pulumi_provider_random_interface::{
    Guest, RandomStringArgs,
};

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl Guest for Component {
    fn create_random_string(args: RandomStringArgs) {
        let r#type = "random:index/randomString:RandomString".to_string();

        let request = RegisterResourceRequest {
            type_: r#type,
            name: args.name,
            object_names: vec!["length".to_string()],
            object: vec![ObjectField { object: &args.length }],
        };

        register(&request);
    }

    fn handle_functions() {
        todo!()
    }
}
