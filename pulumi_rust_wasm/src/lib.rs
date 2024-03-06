use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
// use crate::bindings::exports::component::pulumi_wasm::function_holder::Guest;

pub(crate) mod bindings;
// bindings::export!(Component with_types_in bindings);

pub mod output;
// pub mod pulumi;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<String, Box<dyn Fn(Vec<u8>) -> Vec<u8> + Send>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

struct Component {}

// impl Guest for Component {
//     fn invoke_function(function_name: String, arg: Vec<u8>) -> Vec<u8> {
//         let map = HASHMAP.lock().unwrap();
//         let f = map.get(&function_name).unwrap();
//         f(arg)
//     }
// }
