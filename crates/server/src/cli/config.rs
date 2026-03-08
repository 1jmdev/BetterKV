use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Path, PathBuf};

use betterkv_server::auth::parse_user_directive;
use betterkv_server::config::Config;

use crate::cli::{ConfigInput, Directive};

pub(crate) fn load_config_directives(input: &ConfigInput) -> Result<Vec<Directive>, String> {
    let _trace = profiler::scope("server::main::load_config_directives");
    match input {
        ConfigInput::Path(path) => {
            let mut visited = BTreeSet::new();
            load_config_file(path, &mut visited)
        }
        ConfigInput::Stdin => {
            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .map_err(|err| format!("failed to read config from stdin: {err}"))?;
            parse_config_content(&content)
        }
    }
}

fn load_config_file(
    path: &Path,
    visited: &mut BTreeSet<PathBuf>,
) -> Result<Vec<Directive>, String> {
    let _trace = profiler::scope("server::main::load_config_file");
    let canonical = std::fs::canonicalize(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    if !visited.insert(canonical.clone()) {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&canonical)
        .map_err(|err| format!("failed to read {}: {err}", canonical.display()))?;
    let mut directives = parse_config_content(&content)?;

    let mut expanded = Vec::new();
    for directive in directives.drain(..) {
        if directive.name == "include" {
            for include in directive.values {
                let include_path = if Path::new(&include).is_absolute() {
                    PathBuf::from(include)
                } else {
                    canonical
                        .parent()
                        .unwrap_or_else(|| Path::new("."))
                        .join(include)
                };
                let nested = load_config_file(&include_path, visited)?;
                expanded.extend(nested);
            }
        } else {
            expanded.push(directive);
        }
    }

    Ok(expanded)
}

fn parse_config_content(content: &str) -> Result<Vec<Directive>, String> {
    let _trace = profiler::scope("server::main::parse_config_content");
    let mut directives = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let tokens = trimmed
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        if tokens.is_empty() {
            continue;
        }

        directives.push(Directive {
            name: tokens[0].to_ascii_lowercase(),
            values: tokens[1..].to_vec(),
        });
    }

    Ok(directives)
}

pub(crate) fn apply_directive(
    name: &str,
    values: &[String],
    config: &mut Config,
    ignored: &mut BTreeSet<String>,
) -> Result<(), String> {
    let _trace = profiler::scope("server::main::apply_directive");
    match name {
        "bind" => {
            let value = first_value(name, values)?;
            config.bind = value.to_string();
        }
        "port" => {
            let value = first_value(name, values)?;
            config.port = value
                .parse::<u16>()
                .map_err(|_| format!("invalid port '{value}'"))?;
        }
        "hz" => {
            let value = first_value(name, values)?;
            let hz = value
                .parse::<u64>()
                .map_err(|_| format!("invalid hz value '{value}'"))?;
            if hz > 0 {
                config.sweep_interval_ms = (1000 / hz).max(1);
            }
        }
        "io-threads" => {
            let value = first_value(name, values)?;
            let io_threads = value
                .parse::<usize>()
                .map_err(|_| format!("invalid {name} value '{value}'"))?;
            config.io_threads = io_threads.max(1);
        }
        "shards" => {
            let value = first_value(name, values)?;
            let shards = value
                .parse::<usize>()
                .map_err(|_| format!("invalid {name} value '{value}'"))?;
            config.shards = shards.max(1).next_power_of_two();
        }
        "sweep-interval-ms" => {
            let value = first_value(name, values)?;
            config.sweep_interval_ms = value
                .parse::<u64>()
                .map_err(|_| format!("invalid sweep-interval-ms value '{value}'"))?;
        }
        "loglevel" => {
            let value = first_value(name, values)?;
            config.log_level = value.to_ascii_lowercase();
        }
        "logfile" => {
            let value = first_value(name, values)?;
            if value.eq_ignore_ascii_case("stdout") {
                config.log_file = None;
            } else {
                config.log_file = Some(value.to_string());
            }
        }
        "dir" => {
            let value = first_value(name, values)?;
            config.data_dir = value.to_string();
        }
        "dbfilename" => {
            let value = first_value(name, values)?;
            config.dbfilename = value.to_string();
        }
        "save" => {
            config.snapshot_interval_secs = parse_save_interval(values)?;
        }
        "snapshot-on-shutdown" => {
            let value = first_value(name, values)?;
            config.snapshot_on_shutdown = parse_yes_no(name, value)?;
        }
        "requirepass" => {
            let value = first_value(name, values)?;
            config.requirepass = Some(value.to_string());
        }
        "user" => {
            let user = parse_user_directive(values)?;
            config.user_directives.push(user);
        }
        "appendonly" | "daemonize" | "protected-mode" | "io-threads-do-reads" => {
            let value = first_value(name, values)?;
            parse_yes_no(name, value)?;
            ignored.insert(name.to_string());
        }
        _ => {
            ignored.insert(name.to_string());
        }
    }

    Ok(())
}

fn first_value<'a>(name: &str, values: &'a [String]) -> Result<&'a str, String> {
    let _trace = profiler::scope("server::main::first_value");
    values
        .first()
        .map(String::as_str)
        .ok_or_else(|| format!("directive '{name}' requires a value"))
}

fn parse_yes_no(name: &str, value: &str) -> Result<bool, String> {
    let _trace = profiler::scope("server::main::parse_yes_no");
    match value {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(format!("directive '{name}' expects 'yes' or 'no'")),
    }
}

fn parse_save_interval(values: &[String]) -> Result<u64, String> {
    let _trace = profiler::scope("server::main::parse_save_interval");
    if values.is_empty() {
        return Ok(0);
    }

    if values.len() == 1 {
        return values[0]
            .parse::<u64>()
            .map_err(|_| format!("invalid save value '{}'", values[0]));
    }

    if values.len() % 2 != 0 {
        return Err("save expects pairs of <seconds> <changes>".to_string());
    }

    let mut interval: Option<u64> = None;
    for chunk in values.chunks(2) {
        let seconds = chunk[0]
            .parse::<u64>()
            .map_err(|_| format!("invalid save seconds '{}'", chunk[0]))?;
        chunk[1]
            .parse::<u64>()
            .map_err(|_| format!("invalid save changes '{}'", chunk[1]))?;

        if seconds == 0 {
            continue;
        }
        interval = Some(match interval {
            Some(current) => current.min(seconds),
            None => seconds,
        });
    }

    Ok(interval.unwrap_or(0))
}
