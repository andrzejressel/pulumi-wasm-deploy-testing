use crate::bindings::exports::example::service::common_interface;

mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component;

impl common_interface::Guest for Component {
    fn run_common() -> String {
        return "run_common".to_string();
    }
}

