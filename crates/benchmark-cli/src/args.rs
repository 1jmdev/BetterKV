use std::time::{SystemTime, UNIX_EPOCH};

use clap::{ArgAction, Parser};

#[derive(Debug, Clone, Parser)]
#[command(
    name = "betterkv-benchmark",
    version,
    disable_help_flag = true,
    disable_version_flag = true,
    trailing_var_arg = true,
    about = "Redis/Valkey benchmark-compatible load tester for BetterKV"
)]
pub struct Args {
    #[arg(short = 'h', default_value = "127.0.0.1")]
    pub host: String,

    #[arg(short = 'p', default_value_t = 6379)]
    pub port: u16,

    #[arg(short = 's')]
    pub socket: Option<String>,

    #[arg(short = 'a')]
    pub password: Option<String>,

    #[arg(long = "user")]
    pub user: Option<String>,

    #[arg(short = 'u')]
    pub uri: Option<String>,

    #[arg(short = 'c', default_value_t = 50)]
    pub clients: usize,

    #[arg(short = 'n', default_value_t = 100_000)]
    pub requests: u64,

    #[arg(short = 'd', default_value_t = 3)]
    pub data_size: usize,

    #[arg(long = "dbnum", default_value_t = 0)]
    pub dbnum: u32,

    #[arg(short = '3', action = ArgAction::SetTrue)]
    pub resp3: bool,

    #[arg(long = "threads")]
    pub threads: Option<usize>,

    #[arg(long = "cluster", action = ArgAction::SetTrue)]
    pub cluster: bool,

    #[arg(long = "rfr", default_value = "no")]
    pub read_from_replicas: String,

    #[arg(long = "enable-tracking", action = ArgAction::SetTrue)]
    pub enable_tracking: bool,

    #[arg(short = 'k', default_value_t = 1)]
    pub keep_alive: u8,

    #[arg(short = 'r')]
    pub random_keyspace_len: Option<u64>,

    #[arg(short = 'P', default_value_t = 1)]
    pub pipeline: usize,

    #[arg(short = 'q', action = ArgAction::SetTrue)]
    pub quiet: bool,

    #[arg(long = "precision", default_value_t = 3)]
    pub precision: usize,

    #[arg(long = "csv", action = ArgAction::SetTrue)]
    pub csv: bool,

    #[arg(short = 'l', action = ArgAction::SetTrue)]
    pub loop_forever: bool,

    #[arg(short = 't', value_delimiter = ',')]
    pub tests: Vec<String>,

    #[arg(short = 'I', action = ArgAction::SetTrue)]
    pub idle_mode: bool,

    #[arg(short = 'x', action = ArgAction::SetTrue)]
    pub read_last_arg_from_stdin: bool,

    #[arg(long = "seed")]
    pub seed: Option<u64>,

    #[arg(long = "num-functions", default_value_t = 10)]
    pub num_functions: usize,

    #[arg(long = "num-keys-in-fcall", default_value_t = 1)]
    pub num_keys_in_fcall: usize,

    #[arg(long = "tls", action = ArgAction::SetTrue)]
    pub tls: bool,

    #[arg(long = "sni")]
    pub sni: Option<String>,

    #[arg(long = "cacert")]
    pub cacert: Option<String>,

    #[arg(long = "cacertdir")]
    pub cacertdir: Option<String>,

    #[arg(long = "insecure", action = ArgAction::SetTrue)]
    pub insecure: bool,

    #[arg(long = "cert")]
    pub cert: Option<String>,

    #[arg(long = "key")]
    pub key: Option<String>,

    #[arg(long = "tls-ciphers")]
    pub tls_ciphers: Option<String>,

    #[arg(long = "tls-ciphersuites")]
    pub tls_ciphersuites: Option<String>,

    #[arg(long = "strict", action = ArgAction::SetTrue, hide = true)]
    pub strict: bool,

    #[arg(long = "help", action = ArgAction::SetTrue)]
    pub show_help: bool,

    #[arg(long = "version", action = ArgAction::SetTrue)]
    pub show_version: bool,

    #[arg(allow_hyphen_values = true)]
    pub command_args: Vec<String>,
}

impl Args {
    pub fn apply_connection_overrides(&mut self) -> Result<(), String> {
        if let Some(uri) = self.uri.clone() {
            apply_uri(self, &uri)?;
        }
        Ok(())
    }

    pub fn random_seed(&self) -> u64 {
        self.seed.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|value| value.as_nanos() as u64)
                .unwrap_or(0xBAD5_EED)
        })
    }

    pub fn keep_alive_enabled(&self) -> bool {
        self.keep_alive != 0
    }

    pub fn multi_thread_enabled(&self) -> bool {
        self.threads.unwrap_or(1) > 1
    }

    pub fn thread_count(&self) -> usize {
        self.threads.unwrap_or_else(default_threads)
    }
}

pub fn validate_args(args: &Args) -> Result<(), String> {
    if args.clients == 0 {
        return Err("-c must be greater than 0".to_string());
    }
    if args.requests == 0 {
        return Err("-n must be greater than 0".to_string());
    }
    if args.data_size == 0 {
        return Err("-d must be greater than 0".to_string());
    }
    if args.pipeline == 0 {
        return Err("-P must be greater than 0".to_string());
    }
    if args.keep_alive > 1 {
        return Err("-k must be 0 or 1".to_string());
    }
    if matches!(args.random_keyspace_len, Some(0)) {
        return Err("-r must be greater than 0".to_string());
    }
    if args.user.is_some() && args.password.is_none() {
        return Err("--user requires -a".to_string());
    }
    if args.thread_count() == 0 {
        return Err("--threads must be greater than 0".to_string());
    }
    if args.precision > 9 {
        return Err("--precision must be between 0 and 9".to_string());
    }
    validate_supported_features(args)
}

fn validate_supported_features(args: &Args) -> Result<(), String> {
    let unsupported = [
        (args.socket.is_some(), "-s / unix sockets"),
        (args.resp3, "-3 / RESP3"),
        (args.cluster, "--cluster"),
        (args.read_from_replicas != "no", "--rfr"),
        (args.enable_tracking, "--enable-tracking"),
        (args.tls, "--tls"),
        (args.sni.is_some(), "--sni"),
        (args.cacert.is_some(), "--cacert"),
        (args.cacertdir.is_some(), "--cacertdir"),
        (args.insecure, "--insecure"),
        (args.cert.is_some(), "--cert"),
        (args.key.is_some(), "--key"),
        (args.tls_ciphers.is_some(), "--tls-ciphers"),
        (args.tls_ciphersuites.is_some(), "--tls-ciphersuites"),
    ];

    if let Some(name) = unsupported
        .into_iter()
        .find_map(|(enabled, name)| enabled.then_some(name))
    {
        return Err(format!("{name} is not supported yet"));
    }

    Ok(())
}

fn apply_uri(args: &mut Args, raw: &str) -> Result<(), String> {
    let (scheme, rest) = raw
        .split_once("://")
        .ok_or_else(|| format!("invalid URI {raw:?}"))?;

    if scheme != "valkey" && scheme != "redis" {
        return Err(format!("unsupported URI scheme {scheme:?}"));
    }

    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, Some(path)),
        None => (rest, None),
    };

    let (auth_part, host_part) = match authority.rsplit_once('@') {
        Some((auth, host)) => (Some(auth), host),
        None => (None, authority),
    };

    if let Some(auth) = auth_part {
        let (user, password) = match auth.split_once(':') {
            Some((user, password)) => (Some(user), password),
            None => (None, auth),
        };

        if let Some(user) = user {
            args.user = Some(user.to_string());
        }
        args.password = Some(password.to_string());
    }

    let (host, port) = match host_part.rsplit_once(':') {
        Some((host, port)) if !port.is_empty() => {
            let port = port
                .parse::<u16>()
                .map_err(|err| format!("invalid port in URI: {err}"))?;
            (host, port)
        }
        _ => (host_part, 6379),
    };

    if !host.is_empty() {
        args.host = host.to_string();
    }
    args.port = port;

    if let Some(path) = path.filter(|value| !value.is_empty()) {
        args.dbnum = path
            .parse::<u32>()
            .map_err(|err| format!("invalid db number in URI: {err}"))?;
    }

    Ok(())
}

fn default_threads() -> usize {
    std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
}

pub fn help_text(bin_name: &str) -> String {
    format!(
        "Usage: {bin_name} [OPTIONS] [COMMAND ARGS...]\n\nOptions:\n -h <hostname>      Server hostname (default 127.0.0.1)\n -p <port>          Server port (default 6379)\n -s <socket>        Server socket (overrides host and port)\n -a <password>      Password for AUTH\n --user <username>  Used to send ACL style 'AUTH username pass'. Needs -a.\n -u <uri>           Server URI on format valkey://user:password@host:port/dbnum\n                    User, password and dbnum are optional. For authentication\n                    without a username, use username 'default'. For TLS, use\n                    the scheme 'valkeys'.\n -c <clients>       Number of parallel connections (default 50).\n -n <requests>      Total number of requests (default 100000)\n -d <size>          Data size of SET/GET value in bytes (default 3)\n --dbnum <db>       SELECT the specified db number (default 0)\n -3                 Start session in RESP3 protocol mode.\n --threads <num>    Enable multi-thread mode.\n --cluster          Enable cluster mode.\n --rfr <mode>       Enable read from replicas in cluster mode.\n --enable-tracking  Send CLIENT TRACKING on before starting benchmark.\n -k <boolean>       1=keep alive 0=reconnect (default 1)\n -r <keyspacelen>   Use random keys for built-in tests and expand __rand_int__\n                    inside custom command arguments.\n -P <numreq>        Pipeline <numreq> requests. Default 1 (no pipeline).\n -q                 Quiet. Just show query/sec values\n --precision <num>  Number of decimal places to display in latency output (default 3)\n --csv              Output in CSV format\n -l                 Loop. Run the tests forever\n -t <tests>         Only run the comma separated list of tests.\n -I                 Idle mode. Just open N idle connections and wait.\n -x                 Read last argument from STDIN.\n --seed <num>       Set the seed for random number generator. Default seed is based on time.\n --num-functions <num>\n                    Sets the number of functions present in the Lua lib that is\n                    loaded when running the 'function_load' test. (default 10).\n --num-keys-in-fcall <num>\n                    Sets the number of keys passed to FCALL command when running\n                    the 'fcall' test. (default 1)\n --tls              Establish a secure TLS connection.\n --sni <host>       Server name indication for TLS.\n --cacert <file>    CA Certificate file to verify with.\n --cacertdir <dir>  Directory where trusted CA certificates are stored.\n --insecure         Allow insecure TLS connection by skipping cert validation.\n --cert <file>      Client certificate to authenticate with.\n --key <file>       Private key file to authenticate with.\n --tls-ciphers <list> Sets the list of preferred ciphers (TLSv1.2 and below).\n --tls-ciphersuites <list> Sets the list of preferred ciphersuites (TLSv1.3).\n --help             Output this help and exit.\n --version          Output version and exit.\n\nExamples:\n\n Run the benchmark with the default configuration against 127.0.0.1:6379:\n   $ {bin_name}\n\n Use 20 parallel clients, for a total of 100k requests, against 192.168.1.1:\n   $ {bin_name} -h 192.168.1.1 -p 6379 -n 100000 -c 20\n\n Fill 127.0.0.1:6379 with about 1 million keys only using the SET test:\n   $ {bin_name} -t set -n 1000000 -r 100000000\n\n Benchmark 127.0.0.1:6379 for a few commands producing CSV output:\n   $ {bin_name} -t ping_inline,set,get -n 100000 --csv\n\n Benchmark a specific command line:\n   $ {bin_name} -r 10000 -n 10000 eval 'return redis.call(\"ping\")' 0\n\n Fill a list with 10000 random elements:\n   $ {bin_name} -r 10000 -n 10000 lpush mylist __rand_int__\n\n On user specified command lines __rand_int__ is replaced with a random integer\n with a range of values selected by the -r option."
    )
}
