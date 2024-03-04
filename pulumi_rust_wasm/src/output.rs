// use crate::pulumi::Pulumi;
use std::future::IntoFuture;
use std::marker::PhantomData;
use std::rc::Rc;
use uuid::Uuid;
use crate::bindings::component::pulumi_wasm::output_interface;

pub struct Output<T> {
    phantom: PhantomData<T>,
    future: output_interface::Output,
    // pulumi: Rc<Pulumi>,
}

impl<T> Output<T> {
    pub fn new<F: serde::Serialize>(value: &F) -> Self {
        let binding = rmp_serde::to_vec(value).unwrap();
        let arg = binding.as_slice();
        let resource = output_interface::Output::new(arg);
        // let resource = create_output(value).unwrap();
        Output {
            phantom: PhantomData,
            future: resource,
        }
    }

    pub fn map<B, F>(&self, f: F) -> Output<B>
        where
            F: Fn(T) -> B + Send + 'static,
            T: serde::de::DeserializeOwned,
            B: serde::Serialize,
    {
        let f = move |arg: Vec<u8>| {
            let arg = arg.clone();
            let argument = rmp_serde::decode::from_slice(&arg).unwrap();
            let result = f(argument);
            rmp_serde::to_vec(&result).unwrap()
        };

        let uuid = Uuid::now_v7().to_string();
        let mut map = crate::HASHMAP.lock().unwrap();
        map.insert(uuid.clone(), Box::new(f));

        let new_output = self.future.map(uuid.as_str());

        Output {
            phantom: PhantomData,
            future: new_output,
        }

        // crate::bindings::component::pulumi_wasm::output_interface::Output::

        // let resource = self.pulumi.map(self.future, f).unwrap();
        // Output {
        //     phantom: PhantomData,
        // future: resource,
        // pulumi: self.pulumi.clone(),
        // }
    }
    //
    // pub fn get_value(&self) -> Option<T>
    // where
    //     T: serde::de::DeserializeOwned,
    // {
    //     self.pulumi
    //         .get_output(self.future)
    //         .unwrap()
    //         .map(|output| rmp_serde::decode::from_slice(&*output).unwrap())
    // }
}
