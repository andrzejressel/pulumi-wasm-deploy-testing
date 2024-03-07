use anyhow::Error;
use log::{error, info, LevelFilter, warn};
use pulumi_rust::pulumi::Pulumi;

use clap::{arg, Args, Parser, Subcommand};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::json::JsonEncoder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run,
    Plugins
}


#[derive(Debug, Args)]
struct GlobalOpts {
    #[arg(short, long)]
    wasm: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = App::parse();

    println!("Hello, world!");

    let logfile = FileAppender::builder()
        // .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)} - [{l}] [{M}] [{f}:{L}] {m}{n})}")))
        .encoder(Box::new(JsonEncoder::new()))
        .build("output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    info!("TEST");

    let _pulumi_engine_url = std::env::var("PULUMI_ENGINE")?;
    let pulumi_monitor_url = std::env::var("PULUMI_MONITOR")?;

    info!("INFO LOG");
    warn!("WARN LOG");
    error!("ERROR LOG");

    match &args.command {
        Command::Run => {
            let pulumi = Pulumi::create(&args.global_opts.wasm, &Some(pulumi_monitor_url)).await?;
            pulumi.start().await?;
        }
        Command::Plugins => todo!()
    }


    // tokio::time::sleep(Duration::from_secs(1)).await;

    // pulumi_rust::log(pulumi_engine_url.clone(), "TEST 123".into()).await?;
    // let result = pulumi_rust::create_stuff(pulumi_monitor_url).await?;
    // pulumi_rust::log(pulumi_engine_url.clone(), result.into()).await?;

    Ok(())
}
