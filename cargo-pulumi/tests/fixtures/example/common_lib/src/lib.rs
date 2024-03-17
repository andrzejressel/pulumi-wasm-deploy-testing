// mod bindings;

mod bindings {
    wit_bindgen::generate!({
        path: "../service.wit",
        world: "common-lib"
    });
}

pub fn run_common() -> String {
    let result = crate::bindings::example::service::common_interface::run_common();
    return format!("Hello from common-lib: {}", result);
}