use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "betterkv-tester",
    version,
    about = "Run .rtest suites against a BetterKV or Redis-compatible server"
)]
pub struct Args {
    #[arg(value_name = "PATH", default_value = "tests")]
    pub path: PathBuf,
    #[arg(short = 'H', long = "host", default_value = "127.0.0.1")]
    pub host: String,
    #[arg(short = 'p', long = "port", default_value_t = 6379)]
    pub port: u16,
    #[arg(short = 'a', long = "pass")]
    pub password: Option<String>,
    #[arg(long = "user")]
    pub user: Option<String>,
    #[arg(short = 'n', long = "dbnum", default_value_t = 0)]
    pub db: u32,
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,
}
