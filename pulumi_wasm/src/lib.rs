use core::fmt::Debug;
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::ops::Deref;
use prost::Message;
use rmpv::Value;
use bindings::component::pulumi_wasm::external_world;

use crate::bindings::exports::component::pulumi_wasm::function_reverse_callback::{
    FunctionInvocationRequest, FunctionInvocationResult,
};
use crate::bindings::exports::component::pulumi_wasm::output_interface::Output as WasmOutput;
use crate::bindings::exports::component::pulumi_wasm::output_interface::OutputBorrow as WasmOutputBorrow;
use crate::bindings::exports::component::pulumi_wasm::output_interface::{GuestOutput};
use crate::bindings::exports::component::pulumi_wasm::register_interface::{RegisterResourceRequest};
use crate::bindings::exports::component::pulumi_wasm::{
    function_reverse_callback, output_interface, register_interface,
};
use crate::output::{access_map, FunctionId, FunctionSource, OutputContent};
bindings::export!(Component with_types_in bindings);

mod bindings;
mod grpc;
mod output;

struct Component;

impl output_interface::Guest for Component {
    type Output = Output;
}

pub struct Output {
    output: output::OutputContentRefCell,
    tags: Vec<String>,
}

impl Debug for Output {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(()) // TODO: Implement Debug for Output
    }
}

impl GuestOutput for Output {
    fn new(value: Vec<u8>) -> Self {
        let value = rmpv::decode::read_value(&mut value.as_slice()).unwrap();
        let cell = output::create_new(value);
        Output { output: cell, tags: vec![] }
    }

    fn map(&self, function_name: String) -> WasmOutput {
        let function_id = FunctionId::from_string(&function_name);
        let function_source = FunctionSource::from_str("source");
        let output = output::map_external(function_id, function_source, self.output.clone());
        WasmOutput::new(Output { output, tags: vec![] })
    }

    fn get(&self) -> Option<Vec<u8>> {
        let ref_cell = self.output.borrow();
        let content = ref_cell.deref();

        match content {
            OutputContent::Done(v) => {
                let mut vec = vec![];
                rmpv::encode::write_value(&mut vec, v).unwrap();
                Some(vec)
            }
            OutputContent::Mapped(_, _, _) | OutputContent::Func(_, _) | OutputContent::Nothing => {
                None
            }
        }
    }

    fn duplicate(&self) -> WasmOutput {
        WasmOutput::new(Output {
            output: self.output.clone(),
            tags: self.tags.clone(),
        })
    }
}

impl function_reverse_callback::Guest for Component {
    fn get_functions(source: String) -> Vec<FunctionInvocationRequest> {
        let function_source = &FunctionSource::from_string(&source);
        access_map()
            .iter()
            .flat_map(|f| {
                let f1 = &*f.borrow();

                match f1 {
                    OutputContent::Mapped(id, s, prev) if s == function_source => {
                        match &*prev.borrow() {
                            OutputContent::Done(v) => {
                                let mut vec = vec![];
                                rmpv::encode::write_value(&mut vec, v).unwrap();
                                Some(FunctionInvocationRequest {
                                    id: WasmOutput::new(Output { output: f.clone(), tags: vec![] }),
                                    function_id: id.get(),
                                    value: vec,
                                })
                            }
                            OutputContent::Mapped(_, _, _)
                            | OutputContent::Func(_, _)
                            | OutputContent::Nothing => None,
                        }
                    }
                    OutputContent::Mapped(_, _, _)
                    | OutputContent::Func(_, _)
                    | OutputContent::Done(_)
                    | OutputContent::Nothing => None,
                }
            })
            .collect()
    }

    fn set_functions(results: Vec<FunctionInvocationResult>) {
        for x in results {
            let value = rmpv::decode::read_value(&mut x.value.as_slice()).unwrap();
            let borrowed = &x.id.get::<Output>().output;
            borrowed.replace(OutputContent::Done(value));
        }
    }
}

fn messagepack_to_protoc(v: &Value) -> prost_types::Value {
    match v {
        Value::Integer(i) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::NumberValue(i.as_f64().unwrap())),
        },
        _ => todo!()
    }
}

impl register_interface::Guest for Component {
    fn register(request: RegisterResourceRequest) {

        let pairs = request.object_names.iter().zip(request.object.iter());

        let pairs= pairs.map(|(name, object )| {
            let v = match &*object.object.get::<Output>().output.borrow() {
                OutputContent::Done(vec) => messagepack_to_protoc(&vec),
                OutputContent::Mapped(_, _, _) => todo!(),
                OutputContent::Func(_, _) => todo!(),
                OutputContent::Nothing => todo!()
            };

            (name.clone(), v)
        });



        let object = prost_types::Struct {
            fields: BTreeMap::from_iter(pairs)
        };

        let request = grpc::RegisterResourceRequest {
            r#type: request.type_.clone(),
            name: request.name.clone(),
            parent: "".to_string(),
            custom: true,
            object: Some(object),
            protect: false,
            dependencies: vec![],
            provider: "".to_string(),
            property_dependencies: Default::default(),
            delete_before_replace: false,
            version: "".to_string(),
            ignore_changes: vec![],
            accept_secrets: true,
            additional_secret_outputs: vec![],
            alias_ur_ns: vec![],
            import_id: "".to_string(),
            custom_timeouts: None,
            delete_before_replace_defined: false,
            supports_partial_values: false,
            remote: false,
            accept_resources: false,
            providers: Default::default(),
            replace_on_changes: vec![],
            plugin_download_url: "".to_string(),
            plugin_checksums: Default::default(),
            retain_on_delete: false,
            aliases: vec![],
            deleted_with: "".to_string(),
            alias_specs: false,
            source_position: None,
        };

        let vec_request = request.encode_to_vec();

        external_world::register_resource(vec_request.as_slice());
    }
}
