use anyhow::Context;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings {
    wit_bindgen::generate!({
        // the name of the world in the `*.wit` input file
        world: "pulumi-provider-random-client",
        path: "../../wits/world.wit"
    });
}

pub mod random;
