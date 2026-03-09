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
    pub kind: BenchKind,
    pub clients: usize,
    pub requests: u64,
    pub warmup_requests: u64,
    pub data_size: usize,
    pub pipeline: usize,
    pub random_keys: bool,
    pub keyspace: u64,
    pub key_prefix: String,
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

pub fn resolve_workload(args: &Args) -> Result<Vec<BenchRun>, String> {
    if args.tests.is_empty() {
        return Ok(TESTS
            .iter()
            .copied()
            .map(|spec| run_from_test(args, spec))
            .collect());
    }

    args.tests
        .iter()
        .map(|raw| {
            find_test(raw)
                .map(|spec| run_from_test(args, spec))
                .ok_or_else(|| unknown_test_error(raw))
        })
        .collect()
}

pub fn tests() -> &'static [BenchSpec] {
    TESTS
}

fn run_from_test(args: &Args, spec: BenchSpec) -> BenchRun {
    BenchRun {
        name: spec.name.to_string(),
        kind: spec.kind,
        clients: args.clients,
        requests: args.requests,
        warmup_requests: args.warmup_requests(args.requests),
        data_size: args.data_size,
        pipeline: args.pipeline,
        random_keys: args.random_keys(),
        keyspace: args.keyspace(),
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
