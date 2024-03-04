use crate::bindings::component::pulumi_wasm::output_interface::create_struct;
use crate::bindings::component::pulumi_wasm::register_interface::{
    register, RegisterResourceRequest,
};
use crate::bindings::exports::component::pulumi_wasm::pulumi_provider_random_interface::{
    Guest, Output, RandomStringArgs,
};

mod bindings;

struct Component {}

impl Guest for Component {
    fn create_random_string(args: RandomStringArgs) {
        let r#type = "random:index/randomString:RandomString".to_string();

        let handle;
        let length = match args.length {
            Ok(length) => {
                handle = Output::new(rmp_serde::to_vec(&length).unwrap().as_slice());
                &handle
            }
            Err(output) => output,
        };

        let object = create_struct(vec![("length".to_string(), length)].as_slice());

        let request = RegisterResourceRequest {
            type_: r#type,
            name: args.name,
            object: &object,
        };

        register(request);
    }

    fn handle_functions() {
        todo!()
    }
}
