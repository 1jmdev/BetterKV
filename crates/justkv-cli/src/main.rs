use clap::Parser;
use justkv::config::Config;

#[derive(Debug, Parser)]
#[command(name = "justkv")]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    bind: String,
    #[arg(long, default_value_t = 6379)]
    port: u16,
    #[arg(long)]
    shards: Option<usize>,
    #[arg(long, default_value_t = 250)]
    sweep_interval_ms: u64,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cli = Cli::parse();
    let mut config = Config::default();
    config.bind = cli.bind;
    config.port = cli.port;
    config.sweep_interval_ms = cli.sweep_interval_ms;
    if let Some(shards) = cli.shards {
        config.shards = shards;
    }

    if let Err(err) = justkv::run(config).await {
        eprintln!("server error: {err}");
        std::process::exit(1);
    }
}
