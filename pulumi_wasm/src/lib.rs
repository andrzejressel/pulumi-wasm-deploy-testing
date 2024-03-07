use core::fmt::Debug;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Formatter;
use std::ops::Deref;
use futures::SinkExt;
use lazy_static::lazy_static;
use log::{error, info, log};
use prost::Message;
use prost_types::value::Kind;
use rmpv::{Utf8String, Value};
use bindings::component::pulumi_wasm::external_world;
use crate::bindings::component::pulumi_wasm::external_world::is_in_preview;
use crate::bindings::component::pulumi_wasm::log::log;

use crate::bindings::exports::component::pulumi_wasm::function_reverse_callback::{
    FunctionInvocationRequest, FunctionInvocationResult,
};
use crate::bindings::exports::component::pulumi_wasm::output_interface::Output as WasmOutput;
use crate::bindings::exports::component::pulumi_wasm::output_interface::OutputBorrow as WasmOutputBorrow;
use crate::bindings::exports::component::pulumi_wasm::output_interface::{GuestOutput};
use crate::bindings::exports::component::pulumi_wasm::register_interface::{ObjectField, RegisterResourceRequest};
use crate::bindings::exports::component::pulumi_wasm::{
    function_reverse_callback, output_interface, register_interface,
};
use crate::grpc::register_resource_request;
use crate::output::{access_map, FunctionId, FunctionSource, OutputContent};
bindings::export!(Component with_types_in bindings);

mod bindings;
mod grpc;
mod output;

struct Component;

impl output_interface::Guest for Component {
    type Output = Output;

    fn describe_outputs() -> String {
        wasm_common::setup_logger();
        let outputs = access_map()
            .iter()
            .map(|o| {
                let ref_cell = o.borrow();
                let content = ref_cell.deref();
                content.tpe()
                // o.tpe
            })
            .collect::<Vec<_>>();

        format!("{:?}", outputs)
    }

    fn non_done_exists() -> bool {
        wasm_common::setup_logger();
        for o in access_map() {
            let ref_cell = o.borrow();
            let content = ref_cell.deref();
            match content {
                OutputContent::Done(_) => {}
                OutputContent::Mapped(_, _, _) => return true,
                OutputContent::Func(_, _) => return true,
                OutputContent::Nothing => return true
            }
        }
        false
    }

    fn combine_outputs() -> bool {
        wasm_common::setup_logger();
        let outputs = access_map();
        let mut changed = false;

        outputs.iter().for_each(|o| {
            let ref_cell = o.borrow();
            let content = ref_cell.deref();

            let new_value = match content {
                OutputContent::Func(refcells, f) => {
                    info!("Found func");
                    let data = refcells.iter().flat_map(|r| {
                        let ref_cell = r.borrow();
                        let content = ref_cell.deref();
                        match content {
                            OutputContent::Done(v) => {
                                Some(v.clone())
                            }
                            OutputContent::Mapped(_, _, _) | OutputContent::Func(_, _) | OutputContent::Nothing => None
                        }
                    }).collect::<Vec<_>>();

                    if (data.len() == refcells.len()) {
                        info!("Map");
                        Some(f(data))
                    } else {
                        info!("Cannot map");
                        None
                    }
                }
                OutputContent::Done(_) => None,
                OutputContent::Mapped(_, _, _) => None,
                OutputContent::Nothing => None
            };

            drop(ref_cell);
            match new_value {
                None => {}
                Some(i) => {
                    changed = true;
                    o.replace(OutputContent::Done(i));
                }
            };
        });

        changed
    }
}

#[derive(Clone)]
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
        wasm_common::setup_logger();
        let value = rmpv::decode::read_value(&mut value.as_slice()).unwrap();
        let cell = output::create_new(value);
        Output { output: cell, tags: vec![] }
    }

    fn map(&self, function_name: String) -> WasmOutput {
        wasm_common::setup_logger();
        let function_id = FunctionId::from_string(&function_name);
        let function_source = FunctionSource::from_str("source");
        let output = output::map_external(function_id, function_source, self.output.clone());
        WasmOutput::new(Output { output, tags: vec![] })
    }

    fn get(&self) -> Option<Vec<u8>> {
        wasm_common::setup_logger();
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

    fn get_field(&self, field: String) -> WasmOutput {
        wasm_common::setup_logger();

        info!("Getting field [{field}] from Output [TODO]");

        let o = output::map_internal(vec![self.output.clone()], move |v| {
            let v = v[0].clone();
            info!("Value is [{v}]");

            let v = match v {
                Value::Map(m) => {
                    let key = Value::String(Utf8String::from(field.clone()));
                    let maybe_value = m.iter().find(|(k, _)| k == &key ).map(|(_, v)| v.clone());//.unwrap_or(Value::Nil)
                    match maybe_value {
                        None => if (is_in_preview()) { Value::Nil } else { todo!() }
                        Some(v) => v
                    }
                }
                Value::Nil => todo!(),
                Value::Boolean(_) => todo!(),
                Value::Integer(_) => todo!(),
                Value::F32(_) => todo!(),
                Value::F64(_) => todo!(),
                Value::String(_) => todo!(),
                Value::Binary(_) => todo!(),
                Value::Array(_) => todo!(),
                Value::Ext(_, _) => todo!(),
            };

            info!("Result is [{v}]");

            v
        });

        WasmOutput::new(Output { output: o, tags: vec![] })
    }

    fn get_type(&self) -> String {
        wasm_common::setup_logger();
        let ref_cell = self.output.borrow();
        let content = ref_cell.deref();
        match content {
            OutputContent::Done(_) => "Done",
            OutputContent::Mapped(_, _, _) => "Mapped",
            OutputContent::Func(_, _) => "Func",
            OutputContent::Nothing => "Nothing"
        }.to_string()
    }

    fn duplicate(&self) -> WasmOutput {
        wasm_common::setup_logger();
        WasmOutput::new(Output {
            output: self.output.clone(),
            tags: self.tags.clone(),
        })
    }
}

impl function_reverse_callback::Guest for Component {
    fn get_functions(source: String) -> Vec<FunctionInvocationRequest> {
        wasm_common::setup_logger();
        let function_source = &FunctionSource::from_string(&source);
        access_map()
            .iter()
            .flat_map(|f| {
                let f1 = &*f.borrow();

                match f1 {
                    OutputContent::Mapped(id, s, prev) if s == function_source => {
                        info!("Found mapped");
                        match &*prev.borrow() {
                            OutputContent::Done(v) => {
                                info!("Found value");
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
                            | OutputContent::Nothing => {
                                info!("Value not found");
                                None
                            },
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
        wasm_common::setup_logger();
        for x in results {
            let value = rmpv::decode::read_value(&mut x.value.as_slice()).unwrap();
            let borrowed = &x.id.get::<Output>().output;
            borrowed.replace(OutputContent::Done(value));
        }
    }
}

fn messagepack_to_protoc(v: &Value) -> prost_types::Value {
    info!("Converting value [{v}] to protoc value");
    let result = match v {
        Value::Integer(i) => prost_types::Value {
            kind: Option::from(prost_types::value::Kind::NumberValue(i.as_f64().unwrap())),
        },
        _ => {
            error!("Cannot convert [{v}]");
            todo!("Cannot convert [{v}]")
        }
    };
    info!("Result: [{result:?}]");
    result
}

impl register_interface::Guest for Component {
    fn register(request: RegisterResourceRequest) -> WasmOutput {
        wasm_common::setup_logger();

        let values = request.object.iter().map(|o| o.value.get::<Output>().output.clone()).collect::<Vec<_>>();
        let names = request.object.iter().map(|o| o.name.clone()).collect::<Vec<_>>();

        let new_output = output::map_internal(values, move |v| {

            info!("Converting values [{v:?}] with names [{names:?}]");

            let pairs = names.iter().zip(v.iter()).map(|(name, value)| {
                let v = messagepack_to_protoc(value);
                (name.clone(), v)
            }).collect::<Vec<_>>();

            let object = prost_types::Struct {
                fields: BTreeMap::from_iter(pairs)
            };

            info!("Resulting object: [{object:?}]");

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
                // property_dependencies: HashMap::from(
                //     [("value".to_string(), register_resource_request::PropertyDependencies { urns: vec!["test".to_string()] })]
                // ),
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

            let result_vec = external_world::register_resource(vec_request.as_slice());

            let response = grpc::RegisterResourceResponse::decode(&mut result_vec.as_slice()).unwrap();

            info!("Response: [{response:?}]");

            let result = if (!is_in_preview()) {
                let result = response.object.unwrap().fields.get("result").unwrap().clone().kind.unwrap();

                match result {
                    Kind::NullValue(_) => todo!(),
                    Kind::NumberValue(_) => todo!(),
                    Kind::StringValue(s) => Value::String(s.into()),
                    Kind::BoolValue(_) => todo!(),
                    Kind::StructValue(_) => todo!(),
                    Kind::ListValue(_) => todo!()
                }
            } else {
                Value::Nil
            };

            // FIXME
            Value::Map(
                vec!((Value::from("result"), result))
            )

        });

        WasmOutput::new(Output { output: new_output, tags: vec![] })

    }
}
