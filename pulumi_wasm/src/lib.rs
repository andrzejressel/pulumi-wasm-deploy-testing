use core::fmt::Debug;
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::ops::Deref;
use prost::Message;

use crate::bindings::exports::component::pulumi_wasm::function_reverse_callback::{
    FunctionInvocationRequest, FunctionInvocationResult,
};
use crate::bindings::exports::component::pulumi_wasm::output_interface::{GuestOutput, OwnOutput};
use crate::bindings::exports::component::pulumi_wasm::register_interface::RegisterResourceRequest;
use crate::bindings::exports::component::pulumi_wasm::{
    function_reverse_callback, output_interface, register_interface,
};
use crate::output::OutputContent::Done;
use crate::output::{access_map, FunctionId, FunctionSource, OutputContent};

mod bindings;
mod grpc;
mod output;

struct Component;

impl output_interface::Guest for Component {
    fn create_struct(_fields: Vec<(String, &Output)>) -> OwnOutput {
        //FIXME
        let cell = output::create_nothing();
        OwnOutput::new(Output { output: cell })
        // todo!()
        // let mut field_names = vec![];
        // let mut field_values = vec![];
        //
        // for (string, output) in fields {
        //     field_names.push(string);
        //     field_values.push(output.output.future.clone())
        // }
        //
        // let all_futures = join_all(field_values);
        //
        // let f = all_futures.map(move |vec_of_vecs| {
        //     let mut map = vec![];
        //     let mut all_known = true;
        //
        //     // let mut object = rmpv::Value::Map();
        //     for (value, name) in vec_of_vecs.iter().zip(field_names.clone()) {
        //         let name_field = Value::String(name.into());
        //         let value_field = value.deref().clone();
        //
        //         match value_field {
        //             OutputValue::Known(known) => map.push((name_field, known)),
        //             OutputValue::Unknown() => {
        //                 return Arc::new(OutputValue::Unknown());
        //             }
        //         }
        //     }
        //
        //     Arc::new(OutputValue::Known(Value::Map(map)))
        // });
        //
        // let f2: Pin<Box<dyn Future<Output = Arc<OutputValue>>>> = Box::pin(f);
        //
        // let output_wrapper = OutputWrapper {
        //     future: f2.shared(),
        //     tags: vec![],
        // };
        //
        // OwnOutput::new(Output {
        //     output: output_wrapper,
        // })
    }
}

pub struct Output {
    output: output::OutputContentRefCell,
    // output: OutputWrapper,
}

impl Debug for Output {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl GuestOutput for Output {
    fn new(value: Vec<u8>) -> Self {
        let value = rmpv::decode::read_value(&mut value.as_slice()).unwrap();
        let cell = output::create_new(value);
        Output { output: cell }
    }

    fn map(&self, function_name: String) -> OwnOutput {
        let function_id = FunctionId(function_name);
        let function_source = FunctionSource("source".to_string());
        let output = output::map_external(function_id, function_source, self.output.clone());
        OwnOutput::new(Output { output })
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
    // fn new(value: Vec<u8>) -> Self {
    // let value = rmpv::decode::read_value(&mut value.as_slice()).unwrap();
    // let output = create_output(value);
    // return Self { output };
    // }
    //
    // fn map(&self, function_name: String) -> OwnOutput {
    //     let future = self.output.future.clone();
    //
    //     let new_future = map_to_future(future.map(move |f| {
    //         let f = f.deref();
    //         Arc::new(match f {
    //             OutputValue::Known(v) => {
    //                 let mut vec = vec![];
    //                 rmpv::encode::write_value(&mut vec, v).unwrap();
    //                 let mut vec = invoke_function(
    //                     function_name.as_str(),
    //                     &*vec,
    //                 );
    //                 let value = rmpv::decode::read_value(&mut vec.as_slice()).unwrap();
    //                 OutputValue::Known(value)
    //             }
    //             OutputValue::Unknown() => OutputValue::Unknown(),
    //         })
    //     }))
    //     .shared();
    //
    //     let new_output = OutputWrapper {
    //         future: new_future,
    //         tags: self.output.tags.clone(),
    //     };
    //
    //     OwnOutput::new(Output { output: new_output })
    // }
    //
    // fn get(&self) -> Option<Vec<u8>> {
    //     let future = self.output.future.clone();
    //
    //     let result = block_on(future);
    //
    //     let result = &*result;
    //
    //     match result {
    //         OutputValue::Known(v) => {
    //             let mut vec = vec![];
    //             rmpv::encode::write_value(&mut vec, v).unwrap();
    //             Some(vec)
    //         }
    //         OutputValue::Unknown() => None,
    //     }
    // }
}

impl function_reverse_callback::Guest for Component {
    fn get_functions(source: String) -> Vec<FunctionInvocationRequest> {
        access_map()
            .iter()
            .flat_map(|f| {
                let f1 = &*f.borrow();

                match f1 {
                    OutputContent::Mapped(id, s, prev) if s == &FunctionSource(source.clone()) => {
                        match &*prev.borrow() {
                            OutputContent::Done(v) => {
                                println!("DONE");
                                let mut vec = vec![];
                                rmpv::encode::write_value(&mut vec, v).unwrap();
                                Some(FunctionInvocationRequest {
                                    id: OwnOutput::new(Output { output: f.clone() }),
                                    function_id: id.0.clone(),
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
            let borrowed = &x.id.output;
            borrowed.replace(Done(value));
        }
    }
}

impl register_interface::Guest for Component {
    fn register(request: RegisterResourceRequest) {
        let object = prost_types::Struct {
            fields: BTreeMap::from([
                (
                    "length".to_string(),
                    prost_types::Value {
                        kind: Option::from(prost_types::value::Kind::NumberValue(10.into())),
                    },
                ),
                (
                    "minNumeric".to_string(),
                    prost_types::Value {
                        kind: Option::from(prost_types::value::Kind::NumberValue(5.into())),
                    },
                ),
            ]),
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

        crate::bindings::component::pulumi_wasm::external_world::register_resource(
            vec_request.as_slice(),
        );
    }
}
