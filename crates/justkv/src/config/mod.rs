#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,
    pub port: u16,
    pub shards: usize,
    pub sweep_interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1".to_string(),
            port: 6379,
            shards: default_shards(),
            sweep_interval_ms: 250,
        }
    }
}

impl Config {
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
