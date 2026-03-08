use crate::args::Args;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchKind {
    PingInline,
    PingMbulk,
    Set,
    Get,
    Incr,
    Lpush,
    Rpush,
    Lpop,
    Rpop,
    Sadd,
    Hset,
    Spop,
    Zadd,
    ZpopMin,
    Lrange100,
    Lrange300,
    Lrange500,
    Lrange600,
    Mset,
    Custom,
}

#[derive(Clone, Copy, Debug)]
pub struct BenchSpec {
    pub key: &'static str,
    pub name: &'static str,
    pub kind: BenchKind,
}

#[derive(Clone, Debug)]
pub struct CommandTemplate {
    pub parts: Vec<ArgTemplate>,
}

#[derive(Clone, Debug)]
pub enum ArgTemplate {
    Literal(Vec<u8>),
    RandomInt,
}

#[derive(Clone, Debug)]
pub struct BenchRun {
    pub name: String,
    pub kind: BenchKind,
    pub clients: usize,
    pub requests: u64,
    pub data_size: usize,
    pub pipeline: usize,
    pub random_keyspace_len: Option<u64>,
    pub dbnum: u32,
    pub keep_alive: bool,
    pub key_prefix: String,
    pub seed: u64,
    pub command: Option<CommandTemplate>,
}

const TESTS: &[BenchSpec] = &[
    bench("ping_inline", "PING_INLINE", BenchKind::PingInline),
    bench("ping_mbulk", "PING_MBULK", BenchKind::PingMbulk),
    bench("set", "SET", BenchKind::Set),
    bench("get", "GET", BenchKind::Get),
    bench("incr", "INCR", BenchKind::Incr),
    bench("lpush", "LPUSH", BenchKind::Lpush),
    bench("rpush", "RPUSH", BenchKind::Rpush),
    bench("lpop", "LPOP", BenchKind::Lpop),
    bench("rpop", "RPOP", BenchKind::Rpop),
    bench("sadd", "SADD", BenchKind::Sadd),
    bench("hset", "HSET", BenchKind::Hset),
    bench("spop", "SPOP", BenchKind::Spop),
    bench("zadd", "ZADD", BenchKind::Zadd),
    bench("zpopmin", "ZPOPMIN", BenchKind::ZpopMin),
    bench("lrange_100", "LRANGE_100", BenchKind::Lrange100),
    bench("lrange_300", "LRANGE_300", BenchKind::Lrange300),
    bench("lrange_500", "LRANGE_500", BenchKind::Lrange500),
    bench("lrange_600", "LRANGE_600", BenchKind::Lrange600),
    bench("mset", "MSET", BenchKind::Mset),
];

pub fn resolve_workload(
    args: &Args,
    stdin_last_arg: Option<Vec<u8>>,
) -> Result<Vec<BenchRun>, String> {
    if !args.command_args.is_empty() {
        return Ok(vec![BenchRun {
            name: args.command_args[0].to_ascii_uppercase(),
            kind: BenchKind::Custom,
            clients: args.clients,
            requests: args.requests,
            data_size: args.data_size,
            pipeline: args.pipeline,
            random_keyspace_len: args.random_keyspace_len,
            dbnum: args.dbnum,
            keep_alive: args.keep_alive_enabled(),
            key_prefix: "betterkv-benchmark".to_string(),
            seed: args.random_seed(),
            command: Some(build_custom_command(args, stdin_last_arg)?),
        }]);
    }

    let selected = if args.tests.is_empty() {
        tests().to_vec()
    } else {
        args.tests
            .iter()
            .map(|name| find_test(name).ok_or_else(|| unknown_test_error(name)))
            .collect::<Result<Vec<_>, _>>()?
    };

    Ok(selected
        .into_iter()
        .map(|spec| BenchRun {
            name: spec.name.to_string(),
            kind: spec.kind,
            clients: args.clients,
            requests: args.requests,
            data_size: args.data_size,
            pipeline: args.pipeline,
            random_keyspace_len: args.random_keyspace_len,
            dbnum: args.dbnum,
            keep_alive: args.keep_alive_enabled(),
            key_prefix: "betterkv-benchmark".to_string(),
            seed: args.random_seed(),
            command: None,
        })
        .collect())
}

pub fn tests() -> &'static [BenchSpec] {
    TESTS
}

fn build_custom_command(
    args: &Args,
    stdin_last_arg: Option<Vec<u8>>,
) -> Result<CommandTemplate, String> {
    let mut raw = args.command_args.clone();
    if args.read_last_arg_from_stdin {
        let value = stdin_last_arg
            .ok_or_else(|| "-x was provided but no STDIN data was read".to_string())?;
        if raw.is_empty() {
            return Err("-x requires a command".to_string());
        }
        raw.push(String::from_utf8_lossy(&value).into_owned());
    }

    let parts = raw
        .into_iter()
        .map(|arg| {
            if arg == "__rand_int__" {
                ArgTemplate::RandomInt
            } else {
                ArgTemplate::Literal(arg.into_bytes())
            }
        })
        .collect();

    Ok(CommandTemplate { parts })
}

fn find_test(input: &str) -> Option<BenchSpec> {
    let normalized = normalize_name(input);
    TESTS
        .iter()
        .copied()
        .find(|spec| normalize_name(spec.key) == normalized)
}

fn normalize_name(input: &str) -> String {
    input
        .trim()
        .to_ascii_lowercase()
        .replace([' ', '-', '_'], "")
}

fn unknown_test_error(raw: &str) -> String {
    let supported = TESTS
        .iter()
        .map(|spec| spec.key)
        .collect::<Vec<_>>()
        .join(",");
    format!("unknown test '{raw}', supported tests include: {supported}")
}

const fn bench(key: &'static str, name: &'static str, kind: BenchKind) -> BenchSpec {
    BenchSpec { key, name, kind }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_args() -> Args {
        Args {
            host: "127.0.0.1".to_string(),
            port: 6379,
            socket: None,
            password: None,
            user: None,
            uri: None,
            clients: 20,
            requests: 1_000,
            data_size: 3,
            dbnum: 0,
            resp3: false,
            threads: Some(4),
            cluster: false,
            read_from_replicas: "no".to_string(),
            enable_tracking: false,
            keep_alive: 1,
            random_keyspace_len: Some(500),
            pipeline: 4,
            quiet: false,
            precision: 3,
            csv: false,
            loop_forever: false,
            tests: Vec::new(),
            idle_mode: false,
            read_last_arg_from_stdin: false,
            seed: Some(42),
            num_functions: 10,
            num_keys_in_fcall: 1,
            tls: false,
            sni: None,
            cacert: None,
            cacertdir: None,
            insecure: false,
            cert: None,
            key: None,
            tls_ciphers: None,
            tls_ciphersuites: None,
            strict: false,
            show_help: false,
            show_version: false,
            command_args: Vec::new(),
        }
    }

    #[test]
    fn resolves_default_tests_when_no_selection_is_given() {
        let runs = resolve_workload(&sample_args(), None).expect("resolve workload");
        assert_eq!(runs.len(), TESTS.len());
        assert_eq!(runs[0].name, "PING_INLINE");
    }

    #[test]
    fn resolves_custom_command() {
        let mut args = sample_args();
        args.command_args = vec![
            "lpush".to_string(),
            "mylist".to_string(),
            "__rand_int__".to_string(),
        ];

        let runs = resolve_workload(&args, None).expect("resolve workload");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].name, "LPUSH");
        assert!(matches!(
            runs[0].command.as_ref().unwrap().parts[2],
            ArgTemplate::RandomInt
        ));
    }
}
