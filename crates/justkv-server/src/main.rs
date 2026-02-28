use std::collections::BTreeSet;
use std::path::PathBuf;

use clap::Parser;
use justkv::config::Config;

#[derive(Debug, Parser)]
#[command(name = "justkv-server", version, about = "Redis-compatible JustKV server")]
struct Cli {
    #[arg(index = 1)]
    config_file: Option<PathBuf>,

    #[arg(long)]
    bind: Option<String>,
    #[arg(long)]
    port: Option<u16>,

    #[arg(long, num_args = 2, action = clap::ArgAction::Append)]
    save: Vec<String>,
    #[arg(long)]
    appendonly: Option<String>,
    #[arg(long)]
    daemonize: Option<String>,
    #[arg(long)]
    protected_mode: Option<String>,
    #[arg(long)]
    dir: Option<String>,
    #[arg(long)]
    dbfilename: Option<String>,
    #[arg(long)]
    appendfilename: Option<String>,
    #[arg(long)]
    logfile: Option<String>,
    #[arg(long)]
    loglevel: Option<String>,
    #[arg(long)]
    pidfile: Option<String>,
    #[arg(long)]
    requirepass: Option<String>,
    #[arg(long)]
    maxmemory: Option<String>,
    #[arg(long)]
    maxmemory_policy: Option<String>,
    #[arg(long)]
    timeout: Option<u64>,
    #[arg(long)]
    tcp_keepalive: Option<u64>,
    #[arg(long)]
    unixsocket: Option<String>,
    #[arg(long)]
    unixsocketperm: Option<String>,

    #[arg(long)]
    shards: Option<usize>,
    #[arg(long)]
    sweep_interval_ms: Option<u64>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let cli = Cli::parse();
    let mut config = Config::default();
    let mut ignored = BTreeSet::new();

    if let Some(path) = &cli.config_file {
        if let Err(err) = apply_redis_config_file(path, &mut config, &mut ignored) {
            eprintln!("config error: {err}");
            std::process::exit(1);
        }
    }

    if let Some(bind) = cli.bind {
        config.bind = bind;
    }
    if let Some(port) = cli.port {
        config.port = port;
    }

    if cli.save.len() >= 2 {
        ignored.insert("save");
    }
    if let Some(value) = cli.appendonly {
        if parse_yes_no("appendonly", &value).is_err() {
            eprintln!("config error: appendonly expects 'yes' or 'no'");
            std::process::exit(1);
        }
        ignored.insert("appendonly");
    }
    if let Some(value) = cli.daemonize {
        if parse_yes_no("daemonize", &value).is_err() {
            eprintln!("config error: daemonize expects 'yes' or 'no'");
            std::process::exit(1);
        }
        ignored.insert("daemonize");
    }
    if let Some(value) = cli.protected_mode {
        if parse_yes_no("protected-mode", &value).is_err() {
            eprintln!("config error: protected-mode expects 'yes' or 'no'");
            std::process::exit(1);
        }
        ignored.insert("protected-mode");
    }
    if cli.dir.is_some() {
        ignored.insert("dir");
    }
    if cli.dbfilename.is_some() {
        ignored.insert("dbfilename");
    }
    if cli.appendfilename.is_some() {
        ignored.insert("appendfilename");
    }
    if cli.logfile.is_some() {
        ignored.insert("logfile");
    }
    if cli.loglevel.is_some() {
        ignored.insert("loglevel");
    }
    if cli.pidfile.is_some() {
        ignored.insert("pidfile");
    }
    if cli.requirepass.is_some() {
        ignored.insert("requirepass");
    }
    if cli.maxmemory.is_some() {
        ignored.insert("maxmemory");
    }
    if cli.maxmemory_policy.is_some() {
        ignored.insert("maxmemory-policy");
    }
    if cli.timeout.is_some() {
        ignored.insert("timeout");
    }
    if cli.tcp_keepalive.is_some() {
        ignored.insert("tcp-keepalive");
    }
    if cli.unixsocket.is_some() {
        ignored.insert("unixsocket");
    }
    if cli.unixsocketperm.is_some() {
        ignored.insert("unixsocketperm");
    }

    if let Some(shards) = cli.shards {
        config.shards = shards;
    }
    if let Some(sweep_interval_ms) = cli.sweep_interval_ms {
        config.sweep_interval_ms = sweep_interval_ms;
    }

    if !ignored.is_empty() {
        let names = ignored.into_iter().collect::<Vec<_>>().join(", ");
        eprintln!("justkv-server: accepted but ignored options: {names}");
    }

    if let Err(err) = justkv::run(config).await {
        eprintln!("server error: {err}");
        std::process::exit(1);
    }
}

fn apply_redis_config_file(
    path: &PathBuf,
    config: &mut Config,
    ignored: &mut BTreeSet<&'static str>,
) -> Result<(), String> {
    let content = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;

    for (index, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tokens = line.split_whitespace().collect::<Vec<_>>();
        if tokens.is_empty() {
            continue;
        }

        let key = tokens[0].to_ascii_lowercase();
        let values = &tokens[1..];

        match key.as_str() {
            "bind" => {
                let Some(value) = values.first() else {
                    return Err(format!("{}:{} bind requires a value", path.display(), index + 1));
                };
                config.bind = (*value).to_string();
            }
            "port" => {
                let Some(value) = values.first() else {
                    return Err(format!("{}:{} port requires a value", path.display(), index + 1));
                };
                config.port = value
                    .parse::<u16>()
                    .map_err(|_| format!("{}:{} invalid port '{}'", path.display(), index + 1, value))?;
            }
            "appendonly" => {
                let Some(value) = values.first() else {
                    return Err(format!(
                        "{}:{} appendonly requires 'yes' or 'no'",
                        path.display(),
                        index + 1
                    ));
                };
                parse_yes_no("appendonly", value).map_err(|err| {
                    format!("{}:{} {err}", path.display(), index + 1)
                })?;
                ignored.insert("appendonly");
            }
            "daemonize" => {
                let Some(value) = values.first() else {
                    return Err(format!(
                        "{}:{} daemonize requires 'yes' or 'no'",
                        path.display(),
                        index + 1
                    ));
                };
                parse_yes_no("daemonize", value)
                    .map_err(|err| format!("{}:{} {err}", path.display(), index + 1))?;
                ignored.insert("daemonize");
            }
            "protected-mode" => {
                let Some(value) = values.first() else {
                    return Err(format!(
                        "{}:{} protected-mode requires 'yes' or 'no'",
                        path.display(),
                        index + 1
                    ));
                };
                parse_yes_no("protected-mode", value)
                    .map_err(|err| format!("{}:{} {err}", path.display(), index + 1))?;
                ignored.insert("protected-mode");
            }
            "save" => {
                ignored.insert("save");
            }
            "timeout" => {
                ignored.insert("timeout");
            }
            "tcp-keepalive" => {
                ignored.insert("tcp-keepalive");
            }
            "loglevel" => {
                ignored.insert("loglevel");
            }
            "logfile" => {
                ignored.insert("logfile");
            }
            "dir" => {
                ignored.insert("dir");
            }
            "dbfilename" => {
                ignored.insert("dbfilename");
            }
            "appendfilename" => {
                ignored.insert("appendfilename");
            }
            "pidfile" => {
                ignored.insert("pidfile");
            }
            "requirepass" => {
                ignored.insert("requirepass");
            }
            "maxmemory" => {
                ignored.insert("maxmemory");
            }
            "maxmemory-policy" => {
                ignored.insert("maxmemory-policy");
            }
            "unixsocket" => {
                ignored.insert("unixsocket");
            }
            "unixsocketperm" => {
                ignored.insert("unixsocketperm");
            }
            "hz" => {
                let Some(value) = values.first() else {
                    return Err(format!("{}:{} hz requires a value", path.display(), index + 1));
                };
                let hz = value
                    .parse::<u64>()
                    .map_err(|_| format!("{}:{} invalid hz '{}'", path.display(), index + 1, value))?;
                if hz > 0 {
                    config.sweep_interval_ms = (1000 / hz).max(1);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn parse_yes_no(name: &str, value: &str) -> Result<bool, String> {
    match value {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(format!("{name} expects 'yes' or 'no'")),
    }
}
