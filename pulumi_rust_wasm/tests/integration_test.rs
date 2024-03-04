use crate::pulumi_rust::pulumi::Pulumi;
use anyhow::Error;
use pulumi_rust;
use pulumi_rust::output::Output;

#[test]
fn should_map_value() -> Result<(), Error> {
    let pulumi = Pulumi::create("../target/wasm32-wasi/debug/pulumi_wasm.wasm", &None)?;

    let output: Output<String> = Output::new(pulumi.clone(), &"Hello".to_string());
    let output = output.map(|x| format!("{x} World"));
    let output = output.get_value();

    anyhow::ensure!(output.unwrap() == "Hello World");

    Ok(())
}
