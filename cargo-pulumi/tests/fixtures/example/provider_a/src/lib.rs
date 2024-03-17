use crate::bindings::exports::example::service::a_interface::Guest;

mod bindings;

struct Component;

impl Guest for Component {
    fn run_a() -> String {
        let result = common_lib::run_common();
        return format!("Hello from provider-a: {}", result);
    }
}

bindings::export!(Component with_types_in bindings);