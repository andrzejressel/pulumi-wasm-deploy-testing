use crate::bindings::exports::example::service::main_interface::Guest;

mod bindings;

struct Component;

impl Guest for Component {
    fn main() -> String {
        let res1 = provider_a_lib::run_a();
        let res2 = common_lib::run_common();
        format!("Hello from main: [{}] [{}]", res1, res2)
    }
}

bindings::export!(Component with_types_in bindings);