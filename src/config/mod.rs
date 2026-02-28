use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(name = "valkey")]
pub struct Config {
    #[arg(long, default_value = "127.0.0.1")]
    pub bind: String,
    #[arg(long, default_value_t = 6379)]
    pub port: u16,
    #[arg(long, default_value_t = default_shards())]
    pub shards: usize,
    #[arg(long, default_value_t = 250)]
    pub sweep_interval_ms: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self::parse()
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

fn default_shards() -> usize {
    match std::thread::available_parallelism() {
        Ok(value) => value.get().next_power_of_two(),
        Err(_) => 4,
    }
}
