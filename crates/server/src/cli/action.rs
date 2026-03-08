use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum Action {
    Help,
    Version,
    CheckSystem,
    TestMemory(u64),
    Run(RuntimeArgs),
}

#[derive(Debug, Default)]
pub(crate) struct RuntimeArgs {
    pub config_file: Option<ConfigInput>,
    pub directives: Vec<Directive>,
}

#[derive(Debug)]
pub(crate) enum ConfigInput {
    Path(PathBuf),
    Stdin,
}

#[derive(Debug)]
pub(crate) struct Directive {
    pub name: String,
    pub values: Vec<String>,
}

pub(crate) fn parse_cli_args(args: &[String]) -> Result<Action, String> {
    let _trace = profiler::scope("server::main::parse_cli_args");
    let tail = &args[1..];
    if tail.is_empty() {
        return Ok(Action::Run(RuntimeArgs::default()));
    }

    if tail.len() == 1 {
        let arg = tail[0].as_str();
        if arg == "-h" || arg == "--help" {
            return Ok(Action::Help);
        }
        if arg == "-v" || arg == "--version" {
            return Ok(Action::Version);
        }
        if arg == "--check-system" {
            return Ok(Action::CheckSystem);
        }
    }

    if tail.first().map(String::as_str) == Some("--test-memory") {
        if tail.len() != 2 {
            return Err("--test-memory requires exactly one value".to_string());
        }
        let value = tail[1]
            .parse::<u64>()
            .map_err(|_| format!("invalid --test-memory value '{}'", tail[1]))?;
        return Ok(Action::TestMemory(value));
    }

    let mut runtime = RuntimeArgs::default();
    let mut index = 0;

    if let Some(first) = tail.first() {
        if first == "-" {
            runtime.config_file = Some(ConfigInput::Stdin);
            index = 1;
        } else if !first.starts_with("--") {
            runtime.config_file = Some(ConfigInput::Path(PathBuf::from(first)));
            index = 1;
        }
    }

    while index < tail.len() {
        let token = &tail[index];
        if token == "-" {
            index += 1;
            continue;
        }

        if !token.starts_with("--") {
            return Err(format!("unexpected argument '{token}'"));
        }

        let name = token.trim_start_matches("--");
        if name.is_empty() {
            return Err("invalid empty option".to_string());
        }
        if name == "help" {
            return Ok(Action::Help);
        }
        if name == "version" {
            return Ok(Action::Version);
        }
        if name == "check-system" {
            return Ok(Action::CheckSystem);
        }
        if name == "test-memory" {
            if index + 1 >= tail.len() || tail[index + 1].starts_with("--") {
                return Err("--test-memory requires a value".to_string());
            }
            let value = tail[index + 1]
                .parse::<u64>()
                .map_err(|_| format!("invalid --test-memory value '{}'", tail[index + 1]))?;
            return Ok(Action::TestMemory(value));
        }
        if name == "sentinel" {
            return Err("sentinel mode is not supported".to_string());
        }

        index += 1;
        let mut values = Vec::new();
        while index < tail.len() && !tail[index].starts_with("--") && tail[index] != "-" {
            values.push(tail[index].clone());
            index += 1;
        }

        runtime.directives.push(Directive {
            name: name.to_ascii_lowercase(),
            values,
        });
    }

    Ok(Action::Run(runtime))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_config_file_and_cli_overrides() {
        let _trace = profiler::scope("server::main::parses_config_file_and_cli_overrides");
        let args = vec![
            "betterkv-server".to_string(),
            "./conf/redis.conf".to_string(),
            "--port".to_string(),
            "6380".to_string(),
            "--bind".to_string(),
            "0.0.0.0".to_string(),
        ];

        let action = parse_cli_args(&args).expect("parse args");
        match action {
            Action::Run(runtime) => {
                assert!(matches!(runtime.config_file, Some(ConfigInput::Path(_))));
                assert_eq!(runtime.directives.len(), 2);
                assert_eq!(runtime.directives[0].name, "port");
                assert_eq!(runtime.directives[0].values, vec!["6380"]);
            }
            other => panic!("unexpected action: {other:?}"),
        }
    }

    #[test]
    fn parses_test_memory() {
        let _trace = profiler::scope("server::main::parses_test_memory");
        let args = vec![
            "betterkv-server".to_string(),
            "--test-memory".to_string(),
            "256".to_string(),
        ];

        let action = parse_cli_args(&args).expect("parse args");
        match action {
            Action::TestMemory(value) => assert_eq!(value, 256),
            other => panic!("unexpected action: {other:?}"),
        }
    }

    #[test]
    fn parses_help_short_flag() {
        let _trace = profiler::scope("server::main::parses_help_short_flag");
        let args = vec!["betterkv-server".to_string(), "-h".to_string()];
        let action = parse_cli_args(&args).expect("parse args");
        assert!(matches!(action, Action::Help));
    }
}
