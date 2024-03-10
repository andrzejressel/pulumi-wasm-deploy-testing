extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input};

// #[allow(clippy::all)]
// #[allow(dead_code)]
// #[allow(unused_variables)]
// #[allow(unused_unsafe)]
// mod bindings;

// #[allow(clippy::all)]
// #[allow(dead_code)]
// #[allow(unused_variables)]
// #[allow(unused_unsafe)]
// mod bindings {
//     wit_bindgen::generate!({
//         // the name of the world in the `*.wit` input file
//         world: "pulumi-main-world",
//         path: "../wits/world.wit"
//     });
// }

#[proc_macro_attribute]
pub fn pulumi_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_output = &input_fn.sig.output;

    let expanded = quote! {

        fn #fn_name() #fn_output {
            #fn_block
        }

          #[export_name = "component:pulumi-wasm/pulumi-main@0.1.0#main"]
          unsafe extern "C" fn exported() {

            pulumi_wasm_rust::run(|| {
                #fn_name()
            }).unwrap();
          }
    };

    TokenStream::from(expanded)
}

// mod logger;
//
// static IS_SET : AtomicBool = AtomicBool::new(false);
// static LOGGER: Logger = Logger {};
//
// pub fn setup_logger() {
//     if IS_SET.swap(true, Relaxed) {
//         return;
//     }
//     log::set_logger(&LOGGER).unwrap();
//     log::set_max_level(log::LevelFilter::Trace);
// }