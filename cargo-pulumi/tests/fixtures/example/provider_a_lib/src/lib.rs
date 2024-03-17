// mod bindings;

mod bindings {
    wit_bindgen::generate!({
        path: "../service.wit",
        world: "provider-a-lib"
    });
}

pub fn run_a() -> String {
    let result = crate::bindings::example::service::a_interface::run_a();
    return format!("Hello from provider-a-lib: {}", result);
}