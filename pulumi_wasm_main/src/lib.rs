use crate::bindings::exports::component::pulumi_wasm::pulumi_main::Guest;

use pulumi_rust_wasm::output::Output;
use crate::random::*;

mod bindings;
bindings::export!(Component with_types_in bindings);

mod random;

struct Component {}

impl Guest for Component {
    fn main() {
        let _length: Output<i32> = Output::new(&1234);

        create_random_string(RandomStringArgs {
            name: "test",
            length: 34.into(),
        });
    }
}
