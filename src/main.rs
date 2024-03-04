use anyhow::Error;
use log::{error, info, warn};
use pulumi_rust::pulumi::Pulumi;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    wasm: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    println!("Hello, world!");

    let _pulumi_engine_url = std::env::var("PULUMI_ENGINE")?;
    let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")?;
    // println!("{pulumi_engine_url}");

    // Result::Err(Error::msg(format!("{:?}", std::env::vars().collect::<Vec<_>>())))?;

    // let pulumi_logger = PulumiLogger {
    //     engine_url: pulumi_engine_url.clone(),
    // };

    // pulumi_logger.init()?;

    // debug!("Hello, world!");
    info!("INFO LOG");
    warn!("WARN LOG");
    error!("ERROR LOG");
    // info!("Hello, world!");
    // info!("Hello, world!");

    // Err(anyhow::Error::msg("TEST"))?;

    let pulumi = Pulumi::create(&args.wasm, &Some(pulumi_monitor_url)).await?;

    pulumi.start().await?;

    // tokio::time::sleep(Duration::from_secs(1)).await;

    // pulumi_rust::log(pulumi_engine_url.clone(), "TEST 123".into()).await?;
    // let result = pulumi_rust::create_stuff(pulumi_monitor_url).await?;
    // pulumi_rust::log(pulumi_engine_url.clone(), result.into()).await?;

    Ok(())
}
