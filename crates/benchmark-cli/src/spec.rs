use crate::args::Args;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BenchKind {
    PingInline,
    PingMbulk,
    Echo,
    Set,
    SetNx,
    Get,
    GetSet,
    Mset,
    Mget,
    Del,
    Exists,
    Expire,
    Ttl,
    Incr,
    IncrBy,
    Decr,
    DecrBy,
    Strlen,
    SetRange,
    GetRange,
    Lpush,
    Rpush,
    Lpop,
    Rpop,
    Llen,
    Lrange,
    Sadd,
    Srem,
    Scard,
    Sismember,
    Hset,
    Hget,
    Hgetall,
    Hincrby,
    Zadd,
    Zrem,
    Zcard,
    Zscore,
    Zrank,
    Zrevrank,
    Eval,
    EvalRo,
    EvalSha,
    EvalShaRo,
}

#[derive(Clone, Copy, Debug)]
pub struct BenchSpec {
    pub key: &'static str,
    pub name: &'static str,
    pub kind: BenchKind,
}

#[derive(Clone, Debug)]
pub struct BenchRun {
    pub name: String,
    pub scenario: Option<&'static str>,
    pub kind: BenchKind,
    pub clients: usize,
    pub requests: u64,
    pub data_size: usize,
    pub pipeline: usize,
    pub random_keys: bool,
    pub keyspace: u64,
    pub key_prefix: String,
}

#[derive(Clone, Copy, Debug)]
struct ScenarioStep {
    label: &'static str,
    test: &'static str,
    share: u32,
    clients: Option<usize>,
    pipeline: Option<usize>,
    data_size: Option<usize>,
    random_keys: Option<bool>,
    keyspace: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
pub struct ScenarioSpec {
    pub key: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    steps: &'static [ScenarioStep],
}

const TESTS: &[BenchSpec] = &[
    bench("pinginline", "PING_INLINE", BenchKind::PingInline),
    bench("pingmbulk", "PING_MBULK", BenchKind::PingMbulk),
    bench("echo", "ECHO", BenchKind::Echo),
    bench("set", "SET", BenchKind::Set),
    bench("setnx", "SETNX", BenchKind::SetNx),
    bench("get", "GET", BenchKind::Get),
    bench("getset", "GETSET", BenchKind::GetSet),
    bench("mset", "MSET", BenchKind::Mset),
    bench("mget", "MGET", BenchKind::Mget),
    bench("del", "DEL", BenchKind::Del),
    bench("exists", "EXISTS", BenchKind::Exists),
    bench("expire", "EXPIRE", BenchKind::Expire),
    bench("ttl", "TTL", BenchKind::Ttl),
    bench("incr", "INCR", BenchKind::Incr),
    bench("incrby", "INCRBY", BenchKind::IncrBy),
    bench("decr", "DECR", BenchKind::Decr),
    bench("decrby", "DECRBY", BenchKind::DecrBy),
    bench("strlen", "STRLEN", BenchKind::Strlen),
    bench("setrange", "SETRANGE", BenchKind::SetRange),
    bench("getrange", "GETRANGE", BenchKind::GetRange),
    bench("lpush", "LPUSH", BenchKind::Lpush),
    bench("rpush", "RPUSH", BenchKind::Rpush),
    bench("lpop", "LPOP", BenchKind::Lpop),
    bench("rpop", "RPOP", BenchKind::Rpop),
    bench("llen", "LLEN", BenchKind::Llen),
    bench("lrange", "LRANGE", BenchKind::Lrange),
    bench("sadd", "SADD", BenchKind::Sadd),
    bench("srem", "SREM", BenchKind::Srem),
    bench("scard", "SCARD", BenchKind::Scard),
    bench("sismember", "SISMEMBER", BenchKind::Sismember),
    bench("hset", "HSET", BenchKind::Hset),
    bench("hget", "HGET", BenchKind::Hget),
    bench("hgetall", "HGETALL", BenchKind::Hgetall),
    bench("hincrby", "HINCRBY", BenchKind::Hincrby),
    bench("zadd", "ZADD", BenchKind::Zadd),
    bench("zrem", "ZREM", BenchKind::Zrem),
    bench("zcard", "ZCARD", BenchKind::Zcard),
    bench("zscore", "ZSCORE", BenchKind::Zscore),
    bench("zrank", "ZRANK", BenchKind::Zrank),
    bench("zrevrank", "ZREVRANK", BenchKind::Zrevrank),
    bench("eval", "EVAL", BenchKind::Eval),
    bench("evalro", "EVAL_RO", BenchKind::EvalRo),
    bench("evalsha", "EVALSHA", BenchKind::EvalSha),
    bench("evalsharo", "EVALSHA_RO", BenchKind::EvalShaRo),
];

const SCENARIOS: &[ScenarioSpec] = &[
    scenario(
        "cache_read_heavy",
        "Cache Read Heavy",
        "Hot cache traffic with mostly GET and existence checks.",
        &[
            step(
                "Hot GETs",
                "get",
                70,
                None,
                Some(16),
                None,
                Some(true),
                Some(50_000),
            ),
            step(
                "TTL probes",
                "ttl",
                15,
                None,
                Some(8),
                None,
                Some(true),
                Some(50_000),
            ),
            step(
                "Key existence",
                "exists",
                15,
                None,
                Some(8),
                None,
                Some(true),
                Some(50_000),
            ),
        ],
    ),
    scenario(
        "cache_write_heavy",
        "Cache Write Heavy",
        "Write-biased cache churn with expiring keys.",
        &[
            step(
                "Fresh SET",
                "set",
                55,
                None,
                Some(8),
                Some(128),
                Some(true),
                Some(75_000),
            ),
            step(
                "Conditional SETNX",
                "setnx",
                20,
                None,
                Some(8),
                Some(128),
                Some(true),
                Some(75_000),
            ),
            step(
                "Attach expiry",
                "expire",
                25,
                None,
                Some(4),
                None,
                Some(true),
                Some(75_000),
            ),
        ],
    ),
    scenario(
        "session_store",
        "Session Store",
        "Balanced session reads, refreshes, and occasional cleanup.",
        &[
            step(
                "Session lookup",
                "get",
                55,
                None,
                Some(12),
                Some(256),
                Some(true),
                Some(25_000),
            ),
            step(
                "Session refresh",
                "set",
                30,
                None,
                Some(6),
                Some(256),
                Some(true),
                Some(25_000),
            ),
            step(
                "Session delete",
                "del",
                15,
                None,
                Some(2),
                None,
                Some(true),
                Some(25_000),
            ),
        ],
    ),
    scenario(
        "rate_limiter",
        "Rate Limiter",
        "Counter-heavy workload with frequent increments and TTL checks.",
        &[
            step(
                "Increment counters",
                "incr",
                65,
                None,
                Some(32),
                None,
                Some(true),
                Some(100_000),
            ),
            step(
                "Expiry refresh",
                "expire",
                20,
                None,
                Some(8),
                None,
                Some(true),
                Some(100_000),
            ),
            step(
                "Quota reads",
                "get",
                15,
                None,
                Some(8),
                None,
                Some(true),
                Some(100_000),
            ),
        ],
    ),
    scenario(
        "leaderboard_live",
        "Leaderboard Live",
        "Realtime rank updates mixed with score and rank reads.",
        &[
            step(
                "Score writes",
                "zadd",
                45,
                None,
                Some(8),
                Some(24),
                Some(true),
                Some(10_000),
            ),
            step(
                "Score lookup",
                "zscore",
                30,
                None,
                Some(8),
                Some(24),
                Some(true),
                Some(10_000),
            ),
            step(
                "Rank lookup",
                "zrank",
                25,
                None,
                Some(8),
                Some(24),
                Some(true),
                Some(10_000),
            ),
        ],
    ),
    scenario(
        "leaderboard_maintenance",
        "Leaderboard Maintenance",
        "Background cleanup of sorted sets plus cardinality checks.",
        &[
            step(
                "Prune entries",
                "zrem",
                40,
                None,
                Some(4),
                Some(24),
                Some(true),
                Some(10_000),
            ),
            step(
                "Cardinality scans",
                "zcard",
                35,
                None,
                Some(4),
                None,
                Some(true),
                Some(10_000),
            ),
            step(
                "Reverse rank",
                "zrevrank",
                25,
                None,
                Some(4),
                Some(24),
                Some(true),
                Some(10_000),
            ),
        ],
    ),
    scenario(
        "ecommerce_browse",
        "Ecommerce Browse",
        "Catalog lookups dominated by hashes, ranges, and flags.",
        &[
            step(
                "Product hashes",
                "hgetall",
                40,
                None,
                Some(8),
                Some(512),
                Some(true),
                Some(20_000),
            ),
            step(
                "Inventory flags",
                "sismember",
                25,
                None,
                Some(8),
                Some(32),
                Some(true),
                Some(20_000),
            ),
            step(
                "Price windows",
                "getrange",
                20,
                None,
                Some(8),
                Some(128),
                Some(true),
                Some(20_000),
            ),
            step(
                "Cached pages",
                "mget",
                15,
                None,
                Some(4),
                Some(256),
                Some(true),
                Some(20_000),
            ),
        ],
    ),
    scenario(
        "ecommerce_checkout",
        "Ecommerce Checkout",
        "Cart mutation and stock reservation workflow.",
        &[
            step(
                "Cart writes",
                "hset",
                35,
                None,
                Some(4),
                Some(256),
                Some(true),
                Some(15_000),
            ),
            step(
                "Reservation decrements",
                "decrby",
                30,
                None,
                Some(8),
                None,
                Some(true),
                Some(15_000),
            ),
            step(
                "Cart reads",
                "hget",
                20,
                None,
                Some(4),
                Some(256),
                Some(true),
                Some(15_000),
            ),
            step(
                "Order finalize",
                "del",
                15,
                None,
                Some(2),
                None,
                Some(true),
                Some(15_000),
            ),
        ],
    ),
    scenario(
        "social_feed",
        "Social Feed",
        "Feed fanout model with list appends, trims, and page reads.",
        &[
            step(
                "Feed append",
                "rpush",
                35,
                None,
                Some(8),
                Some(256),
                Some(true),
                Some(30_000),
            ),
            step(
                "Recent feed page",
                "lrange",
                40,
                None,
                Some(8),
                Some(256),
                Some(true),
                Some(30_000),
            ),
            step(
                "Feed size",
                "llen",
                25,
                None,
                Some(4),
                None,
                Some(true),
                Some(30_000),
            ),
        ],
    ),
    scenario(
        "chat_room",
        "Chat Room",
        "Small-message fanout with unread counters and backlog pops.",
        &[
            step(
                "Message append",
                "lpush",
                40,
                None,
                Some(12),
                Some(96),
                Some(true),
                Some(40_000),
            ),
            step(
                "Unread increment",
                "hincrby",
                30,
                None,
                Some(12),
                None,
                Some(true),
                Some(40_000),
            ),
            step(
                "Backlog pop",
                "rpop",
                30,
                None,
                Some(6),
                Some(96),
                Some(true),
                Some(40_000),
            ),
        ],
    ),
    scenario(
        "task_queue",
        "Task Queue",
        "Producer and worker style queue churn.",
        &[
            step(
                "Enqueue",
                "lpush",
                45,
                None,
                Some(16),
                Some(128),
                Some(true),
                Some(50_000),
            ),
            step(
                "Dequeue",
                "rpop",
                35,
                None,
                Some(8),
                Some(128),
                Some(true),
                Some(50_000),
            ),
            step(
                "Queue depth",
                "llen",
                20,
                None,
                Some(4),
                None,
                Some(true),
                Some(50_000),
            ),
        ],
    ),
    scenario(
        "analytics_counters",
        "Analytics Counters",
        "Dense metric updates with periodic reads and batch fetches.",
        &[
            step(
                "Counter increment",
                "incrby",
                55,
                None,
                Some(32),
                None,
                Some(true),
                Some(200_000),
            ),
            step(
                "Counter read",
                "get",
                25,
                None,
                Some(8),
                None,
                Some(true),
                Some(200_000),
            ),
            step(
                "Batch read",
                "mget",
                20,
                None,
                Some(4),
                None,
                Some(true),
                Some(200_000),
            ),
        ],
    ),
    scenario(
        "feature_flags",
        "Feature Flags",
        "Read-mostly flag evaluation with occasional rollouts.",
        &[
            step(
                "Flag lookup",
                "get",
                60,
                None,
                Some(16),
                Some(48),
                Some(true),
                Some(5_000),
            ),
            step(
                "Set membership",
                "sismember",
                25,
                None,
                Some(8),
                Some(48),
                Some(true),
                Some(5_000),
            ),
            step(
                "Flag rollout",
                "set",
                15,
                None,
                Some(4),
                Some(48),
                Some(true),
                Some(5_000),
            ),
        ],
    ),
    scenario(
        "document_store",
        "Document Store",
        "Metadata and body access pattern for medium-size documents.",
        &[
            step(
                "Document write",
                "set",
                30,
                None,
                Some(4),
                Some(1024),
                Some(true),
                Some(12_000),
            ),
            step(
                "Document read",
                "get",
                45,
                None,
                Some(4),
                Some(1024),
                Some(true),
                Some(12_000),
            ),
            step(
                "Metadata hash",
                "hgetall",
                25,
                None,
                Some(4),
                Some(128),
                Some(true),
                Some(12_000),
            ),
        ],
    ),
    scenario(
        "scripting_hot_path",
        "Scripting Hot Path",
        "Server-side script execution for write and read logic.",
        &[
            step(
                "EVAL write",
                "eval",
                35,
                None,
                Some(2),
                Some(64),
                Some(true),
                Some(8_000),
            ),
            step(
                "EVALSHA write",
                "evalsha",
                35,
                None,
                Some(2),
                Some(64),
                Some(true),
                Some(8_000),
            ),
            step(
                "EVALSHA read",
                "evalsharo",
                30,
                None,
                Some(2),
                Some(64),
                Some(true),
                Some(8_000),
            ),
        ],
    ),
    scenario(
        "memtier_mixed",
        "Memtier Mixed",
        "Memtier-style mixed read/write pipeline benchmark.",
        &[
            step(
                "Mixed SET",
                "set",
                30,
                None,
                Some(32),
                Some(256),
                Some(true),
                Some(100_000),
            ),
            step(
                "Mixed GET",
                "get",
                55,
                None,
                Some(32),
                Some(256),
                Some(true),
                Some(100_000),
            ),
            step(
                "Mixed DEL",
                "del",
                15,
                None,
                Some(8),
                None,
                Some(true),
                Some(100_000),
            ),
        ],
    ),
];

pub fn resolve_workload(args: &Args) -> Result<Vec<BenchRun>, String> {
    let mut runs = Vec::new();

    if args.tests.is_empty() && args.scenarios.is_empty() {
        for spec in default_specs() {
            runs.push(run_from_test(args, spec, spec.name.to_string()));
        }
        return Ok(runs);
    }

    for raw in &args.tests {
        let spec = find_test(raw).ok_or_else(|| unknown_test_error(raw))?;
        runs.push(run_from_test(args, spec, spec.name.to_string()));
    }

    for raw in &args.scenarios {
        let scenario = find_scenario(raw).ok_or_else(|| unknown_scenario_error(raw))?;
        runs.extend(expand_scenario(args, scenario));
    }

    Ok(runs)
}

pub fn default_specs() -> Vec<BenchSpec> {
    TESTS.to_vec()
}

pub fn scenarios() -> &'static [ScenarioSpec] {
    SCENARIOS
}

pub fn tests() -> &'static [BenchSpec] {
    TESTS
}

fn expand_scenario(args: &Args, scenario: ScenarioSpec) -> Vec<BenchRun> {
    let total_share: u32 = scenario.steps.iter().map(|step| step.share).sum();
    let mut assigned = 0u64;
    let mut runs = Vec::with_capacity(scenario.steps.len());

    for (index, step) in scenario.steps.iter().enumerate() {
        let requests = if index + 1 == scenario.steps.len() {
            args.requests.saturating_sub(assigned).max(1)
        } else {
            let portion = args.requests.saturating_mul(step.share as u64) / total_share as u64;
            portion.max(1)
        };
        assigned = assigned.saturating_add(requests);

        let spec = find_test(step.test).expect("invalid built-in scenario test");
        runs.push(BenchRun {
            name: format!("{} / {} [{}]", scenario.name, step.label, spec.name),
            scenario: Some(scenario.name),
            kind: spec.kind,
            clients: step.clients.unwrap_or(args.clients),
            requests,
            data_size: step.data_size.unwrap_or(args.data_size),
            pipeline: step.pipeline.unwrap_or(args.pipeline),
            random_keys: step.random_keys.unwrap_or(args.random_keys),
            keyspace: step.keyspace.unwrap_or(args.keyspace),
            key_prefix: format!("{}:{}", args.key_prefix, normalize_name(scenario.key)),
        });
    }

    runs
}

fn run_from_test(args: &Args, spec: BenchSpec, name: String) -> BenchRun {
    BenchRun {
        name,
        scenario: None,
        kind: spec.kind,
        clients: args.clients,
        requests: args.requests,
        data_size: args.data_size,
        pipeline: args.pipeline,
        random_keys: args.random_keys,
        keyspace: args.keyspace,
        key_prefix: args.key_prefix.clone(),
    }
}

fn find_test(input: &str) -> Option<BenchSpec> {
    let key = normalize_name(input);
    TESTS
        .iter()
        .copied()
        .find(|spec| normalize_name(spec.key) == key)
}

fn find_scenario(input: &str) -> Option<ScenarioSpec> {
    let key = normalize_name(input);
    SCENARIOS
        .iter()
        .copied()
        .find(|scenario| normalize_name(scenario.key) == key)
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

fn unknown_scenario_error(raw: &str) -> String {
    let supported = SCENARIOS
        .iter()
        .map(|scenario| scenario.key)
        .collect::<Vec<_>>()
        .join(",");
    format!("unknown scenario '{raw}', supported scenarios include: {supported}")
}

const fn bench(key: &'static str, name: &'static str, kind: BenchKind) -> BenchSpec {
    BenchSpec { key, name, kind }
}

const fn scenario(
    key: &'static str,
    name: &'static str,
    description: &'static str,
    steps: &'static [ScenarioStep],
) -> ScenarioSpec {
    ScenarioSpec {
        key,
        name,
        description,
        steps,
    }
}

const fn step(
    label: &'static str,
    test: &'static str,
    share: u32,
    clients: Option<usize>,
    pipeline: Option<usize>,
    data_size: Option<usize>,
    random_keys: Option<bool>,
    keyspace: Option<u64>,
) -> ScenarioStep {
    ScenarioStep {
        label,
        test,
        share,
        clients,
        pipeline,
        data_size,
        random_keys,
        keyspace,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_args() -> Args {
        Args {
            host: "127.0.0.1".to_string(),
            port: 6379,
            auth: None,
            clients: 20,
            requests: 1_000,
            data_size: 64,
            pipeline: 4,
            tests: Vec::new(),
            scenarios: Vec::new(),
            list_tests: false,
            list_scenarios: false,
            quiet: false,
            csv: false,
            random_keys: false,
            keyspace: 500,
            key_prefix: "bench".to_string(),
            threads: 4,
            strict: false,
            help: None,
            version: None,
        }
    }

    #[test]
    fn resolves_default_tests_when_no_selection_is_given() {
        let runs = resolve_workload(&sample_args()).expect("resolve workload");
        assert_eq!(runs.len(), TESTS.len());
        assert_eq!(runs[0].name, "PING_INLINE");
    }

    #[test]
    fn expands_scenario_into_weighted_steps() {
        let mut args = sample_args();
        args.scenarios = vec!["memtier-mixed".to_string()];

        let runs = resolve_workload(&args).expect("resolve workload");
        assert_eq!(runs.len(), 3);
        assert_eq!(
            runs.iter().map(|run| run.requests).sum::<u64>(),
            args.requests
        );
        assert!(runs[0].name.contains("Memtier Mixed"));
        assert!(runs.iter().all(|run| run.random_keys));
    }
}
