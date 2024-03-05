use std::marker::PhantomData;
use crate::bindings::component::pulumi_wasm::output_interface;
use uuid::Uuid;

pub struct Output<T> {
    phantom: PhantomData<T>,
    future: output_interface::Output,
}

impl <T: serde::Serialize> From<T> for Output<T> {
    fn from(value: T) -> Output<T> {
        Output::new(&value)
    }
}

impl<T> Output<T> {

    pub unsafe fn get_inner(&self) -> &output_interface::Output {
        &self.future
    }

    pub fn new<F: serde::Serialize>(value: &F) -> Self {
        let binding = rmp_serde::to_vec(value).unwrap();
        let arg = binding.as_slice();
        let resource = output_interface::Output::new(arg);
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
    }
}
