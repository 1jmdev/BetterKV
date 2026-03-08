use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use bytes::BytesMut;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::args::Args;
use crate::render::progress_line;
use crate::resp::{
    ExpectedResponse, consume_response, encode_expected_response, encode_resp_parts,
};
use crate::spec::{ArgTemplate, BenchKind, BenchRun, CommandTemplate};

const SETUP_BATCH: usize = 64;
const LIST_ITEM_COUNT: usize = 600;
const MSET_KEYS: usize = 10;

#[derive(Default)]
struct WorkerStats {
    completed: u64,
    latencies_ns: Vec<u64>,
}

pub struct CumulativeBucket {
    pub percent: f64,
    pub latency_ms: f64,
    pub cumulative_count: u64,
}

pub struct BenchResult {
    pub name: String,
    pub requests: u64,
    pub clients: usize,
    pub elapsed_secs: f64,
    pub req_per_sec: f64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
    pub data_size: usize,
    pub keep_alive: bool,
    pub multi_thread: bool,
    samples_ns: Vec<u64>,
    pub cumulative_distribution: Vec<CumulativeBucket>,
}

struct Shared {
    host: String,
    port: u16,
    user: Option<String>,
    password: Option<String>,
    run: BenchRun,
    strict: bool,
}

struct Progress {
    completed: AtomicU64,
    finished: AtomicBool,
}

#[derive(Clone, Copy)]
struct ClientPlan {
    client_id: u64,
    quota: u64,
}

#[derive(Clone, Copy)]
struct RandomSource {
    state: u64,
}

impl RandomSource {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }
}

pub async fn maybe_warn_about_server_config(args: &Args) {
    match try_fetch_server_config(args).await {
        Ok(()) => {}
        Err(_) => eprintln!("WARNING: Could not fetch server CONFIG"),
    }
}

async fn try_fetch_server_config(args: &Args) -> Result<(), String> {
    let addr = format!("{}:{}", args.host, args.port);
    let mut stream = TcpStream::connect(&addr)
        .await
        .map_err(|err| format!("connect {addr}: {err}"))?;
    stream
        .set_nodelay(true)
        .map_err(|err| format!("set_nodelay: {err}"))?;

    let mut parse_buf = BytesMut::with_capacity(1024);
    authenticate_and_select(
        &mut stream,
        &mut parse_buf,
        args.user.as_deref(),
        args.password.as_deref(),
        args.dbnum,
    )
    .await?;
    let payload = encode_resp_parts(&[b"CONFIG", b"GET", b"save"]);
    stream
        .write_all(&payload)
        .await
        .map_err(|err| format!("CONFIG write failed: {err}"))?;
    consume_response(&mut stream, &mut parse_buf, None, None, false).await
}

pub async fn run_single_benchmark(args: &Args, run: BenchRun) -> Result<BenchResult, String> {
    let clients = run.clients.min(run.requests as usize).max(1);
    let base = run.requests / clients as u64;
    let extra = (run.requests % clients as u64) as usize;
    let thread_count = args.thread_count().min(clients).max(1);

    let shared = Arc::new(Shared {
        host: args.host.clone(),
        port: args.port,
        user: args.user.clone(),
        password: args.password.clone(),
        run,
        strict: args.strict,
    });

    let mut shards = vec![Vec::new(); thread_count];
    for client_id in 0..clients {
        let quota = base + u64::from(client_id < extra);
        if quota == 0 {
            continue;
        }
        shards[client_id % thread_count].push(ClientPlan {
            client_id: client_id as u64,
            quota,
        });
    }

    let progress = Arc::new(Progress {
        completed: AtomicU64::new(0),
        finished: AtomicBool::new(false),
    });

    let reporter = if args.quiet || args.csv {
        None
    } else {
        Some(spawn_progress_reporter(
            shared.run.name.clone(),
            shared.run.requests,
            Arc::clone(&progress),
        ))
    };

    let start = Instant::now();
    let mut handles = Vec::with_capacity(thread_count);
    for (thread_index, shard) in shards.into_iter().enumerate() {
        if shard.is_empty() {
            continue;
        }

        let cfg = Arc::clone(&shared);
        let progress = Arc::clone(&progress);
        handles.push(
            thread::Builder::new()
                .name(format!("betterkv-bench-{thread_index}"))
                .spawn(move || run_thread_shard(cfg, shard, progress))
                .map_err(|err| format!("failed to spawn benchmark thread {thread_index}: {err}"))?,
        );
    }

    let mut samples = Vec::new();
    let mut total_completed = 0u64;
    for handle in handles {
        let thread_stats = handle
            .join()
            .map_err(|_| "benchmark thread panicked".to_string())??;
        for stats in thread_stats {
            total_completed += stats.completed;
            samples.extend(stats.latencies_ns);
        }
    }

    progress.finished.store(true, Ordering::Relaxed);
    if let Some(reporter) = reporter {
        let _ = reporter.join();
        eprintln!();
    }

    let elapsed_secs = start.elapsed().as_secs_f64();
    if total_completed == 0 || elapsed_secs == 0.0 {
        return Err("benchmark completed with zero successful requests".to_string());
    }

    samples.sort_unstable();
    let avg_ms = samples.iter().copied().map(ns_to_ms).sum::<f64>() / samples.len() as f64;
    let min_ms = ns_to_ms(samples[0]);
    let p50_ms = percentile_ms(&samples, 50.0);
    let p95_ms = percentile_ms(&samples, 95.0);
    let p99_ms = percentile_ms(&samples, 99.0);
    let max_ms = ns_to_ms(*samples.last().unwrap_or(&0));

    Ok(BenchResult {
        name: shared.run.name.clone(),
        requests: total_completed,
        clients,
        elapsed_secs,
        req_per_sec: total_completed as f64 / elapsed_secs,
        avg_ms,
        min_ms,
        p50_ms,
        p95_ms,
        p99_ms,
        max_ms,
        data_size: shared.run.data_size,
        keep_alive: shared.run.keep_alive,
        multi_thread: args.multi_thread_enabled(),
        cumulative_distribution: build_cumulative_distribution(&samples),
        samples_ns: samples,
    })
}

pub async fn run_idle_mode(args: &Args) -> Result<(), String> {
    let addr = format!("{}:{}", args.host, args.port);
    let mut handles = Vec::with_capacity(args.clients);
    for _ in 0..args.clients {
        let addr = addr.clone();
        let user = args.user.clone();
        let password = args.password.clone();
        let dbnum = args.dbnum;
        handles.push(tokio::spawn(async move {
            let mut stream = TcpStream::connect(&addr)
                .await
                .map_err(|err| format!("connect {addr}: {err}"))?;
            stream
                .set_nodelay(true)
                .map_err(|err| format!("set_nodelay: {err}"))?;
            let mut parse_buf = BytesMut::with_capacity(256);
            authenticate_and_select(
                &mut stream,
                &mut parse_buf,
                user.as_deref(),
                password.as_deref(),
                dbnum,
            )
            .await?;
            tokio::time::sleep(Duration::from_secs(u64::MAX / 4)).await;
            Ok::<(), String>(())
        }));
    }

    for handle in handles {
        handle
            .await
            .map_err(|err| format!("idle worker failed: {err}"))??;
    }

    Ok(())
}

impl BenchResult {
    pub fn latency_for_percentile(&self, percentile: f64) -> f64 {
        percentile_ms(&self.samples_ns, percentile)
    }

    pub fn cumulative_count_for_percentile(&self, percentile: f64) -> u64 {
        if self.samples_ns.is_empty() {
            return 0;
        }
        if percentile <= 0.0 {
            return 1;
        }
        let index = percentile_index(self.samples_ns.len(), percentile);
        (index + 1) as u64
    }
}

fn spawn_progress_reporter(
    name: String,
    total: u64,
    progress: Arc<Progress>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let started = Instant::now();
        while !progress.finished.load(Ordering::Relaxed) {
            let completed = progress.completed.load(Ordering::Relaxed);
            eprint!(
                "\r{}",
                progress_line(&name, completed, total, started.elapsed().as_secs_f64())
            );
            thread::sleep(Duration::from_millis(200));
        }

        let completed = progress.completed.load(Ordering::Relaxed);
        eprint!(
            "\r{}",
            progress_line(&name, completed, total, started.elapsed().as_secs_f64())
        );
    })
}

fn run_thread_shard(
    cfg: Arc<Shared>,
    shard: Vec<ClientPlan>,
    progress: Arc<Progress>,
) -> Result<Vec<WorkerStats>, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| format!("failed to create worker runtime: {err}"))?;

    runtime.block_on(async move {
        let mut handles = Vec::with_capacity(shard.len());
        for plan in shard {
            let worker_cfg = Arc::clone(&cfg);
            let worker_progress = Arc::clone(&progress);
            handles.push(tokio::spawn(async move {
                run_worker(plan.client_id, plan.quota, worker_cfg, worker_progress).await
            }));
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            results.push(
                handle
                    .await
                    .map_err(|err| format!("worker join error: {err}"))??,
            );
        }
        Ok(results)
    })
}

async fn run_worker(
    client_id: u64,
    quota: u64,
    cfg: Arc<Shared>,
    progress: Arc<Progress>,
) -> Result<WorkerStats, String> {
    let mut connection = open_connection(&cfg).await?;
    let value = vec![b'x'; cfg.run.data_size];
    let key_base = format!(
        "{}:{}:{client_id}",
        cfg.run.key_prefix,
        cfg.run.name.to_ascii_lowercase()
    );
    let mut random = RandomSource::new(cfg.run.seed ^ client_id.rotate_left(17));

    setup_connection_state(&mut connection, &cfg.run, key_base.as_bytes(), &value).await?;

    let mut stats = WorkerStats {
        latencies_ns: Vec::with_capacity(quota as usize),
        ..WorkerStats::default()
    };
    let mut remaining = quota;
    while remaining > 0 {
        let batch = remaining.min(cfg.run.pipeline as u64) as usize;
        if !cfg.run.keep_alive && stats.completed > 0 {
            connection = open_connection(&cfg).await?;
            setup_connection_state(&mut connection, &cfg.run, key_base.as_bytes(), &value).await?;
        }

        let request_group =
            build_request_group(&cfg.run, key_base.as_bytes(), &value, batch, &mut random)?;
        let sent_at = Instant::now();
        connection
            .stream
            .write_all(&request_group.payload)
            .await
            .map_err(|err| format!("write failed: {err}"))?;

        let mut pending = VecDeque::from(vec![sent_at; batch]);
        for index in 0..batch {
            let expected = request_group.expected[index].as_ref();
            let encoded = request_group.encoded[index].as_deref();
            consume_response(
                &mut connection.stream,
                &mut connection.parse_buf,
                expected,
                encoded,
                cfg.strict,
            )
            .await?;

            let started = pending.pop_front().expect("pending request timestamp");
            stats
                .latencies_ns
                .push(started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64);
        }

        stats.completed += batch as u64;
        progress
            .completed
            .fetch_add(batch as u64, Ordering::Relaxed);
        remaining -= batch as u64;
    }

    Ok(stats)
}

struct Connection {
    stream: TcpStream,
    parse_buf: BytesMut,
}

async fn open_connection(cfg: &Shared) -> Result<Connection, String> {
    let addr = format!("{}:{}", cfg.host, cfg.port);
    let mut stream = TcpStream::connect(&addr)
        .await
        .map_err(|err| format!("connect {addr}: {err}"))?;
    stream
        .set_nodelay(true)
        .map_err(|err| format!("set_nodelay: {err}"))?;

    let mut parse_buf = BytesMut::with_capacity(8192);
    authenticate_and_select(
        &mut stream,
        &mut parse_buf,
        cfg.user.as_deref(),
        cfg.password.as_deref(),
        cfg.run.dbnum,
    )
    .await?;

    Ok(Connection { stream, parse_buf })
}

async fn authenticate_and_select(
    stream: &mut TcpStream,
    parse_buf: &mut BytesMut,
    user: Option<&str>,
    password: Option<&str>,
    dbnum: u32,
) -> Result<(), String> {
    if let Some(password) = password {
        let auth = match user {
            Some(user) => encode_resp_parts(&[b"AUTH", user.as_bytes(), password.as_bytes()]),
            None => encode_resp_parts(&[b"AUTH", password.as_bytes()]),
        };
        stream
            .write_all(&auth)
            .await
            .map_err(|err| format!("AUTH write failed: {err}"))?;
        consume_response(
            stream,
            parse_buf,
            Some(&ExpectedResponse::Simple("OK")),
            Some(b"+OK\r\n"),
            true,
        )
        .await?;
    }

    if dbnum != 0 {
        let db = dbnum.to_string();
        let select = encode_resp_parts(&[b"SELECT", db.as_bytes()]);
        stream
            .write_all(&select)
            .await
            .map_err(|err| format!("SELECT write failed: {err}"))?;
        consume_response(
            stream,
            parse_buf,
            Some(&ExpectedResponse::Simple("OK")),
            Some(b"+OK\r\n"),
            true,
        )
        .await?;
    }

    Ok(())
}

async fn setup_connection_state(
    connection: &mut Connection,
    run: &BenchRun,
    key_base: &[u8],
    value: &[u8],
) -> Result<(), String> {
    match run.kind {
        BenchKind::Get
        | BenchKind::Lpop
        | BenchKind::Rpop
        | BenchKind::Spop
        | BenchKind::ZpopMin => {
            prime_keyspace(
                &mut connection.stream,
                &mut connection.parse_buf,
                run,
                key_base,
                value,
            )
            .await
        }
        BenchKind::Lrange100
        | BenchKind::Lrange300
        | BenchKind::Lrange500
        | BenchKind::Lrange600 => {
            prime_keyspace(
                &mut connection.stream,
                &mut connection.parse_buf,
                run,
                key_base,
                value,
            )
            .await
        }
        _ => Ok(()),
    }
}

async fn prime_keyspace(
    stream: &mut TcpStream,
    parse_buf: &mut BytesMut,
    run: &BenchRun,
    key_base: &[u8],
    value: &[u8],
) -> Result<(), String> {
    let keyspace = run.random_keyspace_len.unwrap_or(1);
    let mut payload = Vec::new();
    let mut pending = 0usize;
    for slot in 0..keyspace {
        if let Some(command) = build_setup_command(run.kind, key_base, slot, value) {
            payload.extend_from_slice(&command);
            pending += 1;
        }

        if pending == SETUP_BATCH {
            stream
                .write_all(&payload)
                .await
                .map_err(|err| format!("setup write failed: {err}"))?;
            for _ in 0..pending {
                consume_response(stream, parse_buf, None, None, false).await?;
            }
            payload.clear();
            pending = 0;
        }
    }

    if pending > 0 {
        stream
            .write_all(&payload)
            .await
            .map_err(|err| format!("setup write failed: {err}"))?;
        for _ in 0..pending {
            consume_response(stream, parse_buf, None, None, false).await?;
        }
    }

    Ok(())
}

struct RequestGroup {
    payload: Vec<u8>,
    expected: Vec<Option<ExpectedResponse>>,
    encoded: Vec<Option<Vec<u8>>>,
}

fn build_request_group(
    run: &BenchRun,
    key_base: &[u8],
    value: &[u8],
    batch: usize,
    random: &mut RandomSource,
) -> Result<RequestGroup, String> {
    let mut payload = Vec::new();
    let mut expected = Vec::with_capacity(batch);
    let mut encoded = Vec::with_capacity(batch);
    for _ in 0..batch {
        let slot = pick_key_slot(random, run.random_keyspace_len);
        let frame = build_command(run, key_base, slot, value, random)?;
        payload.extend_from_slice(&frame);

        let expected_response = expected_response(run.kind, value);
        encoded.push(
            expected_response
                .as_ref()
                .and_then(encode_expected_response),
        );
        expected.push(expected_response);
    }

    Ok(RequestGroup {
        payload,
        expected,
        encoded,
    })
}

fn pick_key_slot(random: &mut RandomSource, keyspace: Option<u64>) -> u64 {
    match keyspace {
        Some(0) | None => 0,
        Some(1) => 0,
        Some(keyspace) => random.next() % keyspace,
    }
}

fn build_setup_command(
    kind: BenchKind,
    key_base: &[u8],
    slot: u64,
    value: &[u8],
) -> Option<Vec<u8>> {
    let key = make_key(key_base, slot);
    match kind {
        BenchKind::Get => Some(encode_resp_parts(&[b"SET", key.as_slice(), value])),
        BenchKind::Lpop | BenchKind::Rpop => {
            Some(encode_resp_parts(&[b"LPUSH", key.as_slice(), value]))
        }
        BenchKind::Spop => Some(encode_resp_parts(&[b"SADD", key.as_slice(), value])),
        BenchKind::ZpopMin => Some(encode_resp_parts(&[b"ZADD", key.as_slice(), b"1", value])),
        BenchKind::Lrange100
        | BenchKind::Lrange300
        | BenchKind::Lrange500
        | BenchKind::Lrange600 => {
            let mut parts = Vec::with_capacity(2 + LIST_ITEM_COUNT);
            parts.push(b"LPUSH".as_slice());
            parts.push(key.as_slice());
            let list_len = lrange_target(kind);
            let mut items = Vec::with_capacity(list_len);
            for index in 0..list_len {
                items.push(format!("item:{index}").into_bytes());
            }
            for item in &items {
                parts.push(item.as_slice());
            }
            Some(encode_resp_parts(&parts))
        }
        _ => None,
    }
}

fn build_command(
    run: &BenchRun,
    key_base: &[u8],
    slot: u64,
    value: &[u8],
    random: &mut RandomSource,
) -> Result<Vec<u8>, String> {
    Ok(match run.kind {
        BenchKind::PingInline => b"PING\r\n".to_vec(),
        BenchKind::PingMbulk => encode_resp_parts(&[b"PING"]),
        BenchKind::Set => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"SET", key.as_slice(), value])
        }
        BenchKind::Get => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"GET", key.as_slice()])
        }
        BenchKind::Incr => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"INCR", key.as_slice()])
        }
        BenchKind::Lpush => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"LPUSH", key.as_slice(), value])
        }
        BenchKind::Rpush => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"RPUSH", key.as_slice(), value])
        }
        BenchKind::Lpop => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"LPOP", key.as_slice()])
        }
        BenchKind::Rpop => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"RPOP", key.as_slice()])
        }
        BenchKind::Sadd => {
            let key = make_key(key_base, slot);
            let member = if run.random_keyspace_len.is_some() {
                random.next().to_string().into_bytes()
            } else {
                value.to_vec()
            };
            encode_resp_parts(&[b"SADD", key.as_slice(), member.as_slice()])
        }
        BenchKind::Hset => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"HSET", key.as_slice(), b"field", value])
        }
        BenchKind::Spop => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"SPOP", key.as_slice()])
        }
        BenchKind::Zadd => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"ZADD", key.as_slice(), b"1", value])
        }
        BenchKind::ZpopMin => {
            let key = make_key(key_base, slot);
            encode_resp_parts(&[b"ZPOPMIN", key.as_slice()])
        }
        BenchKind::Lrange100
        | BenchKind::Lrange300
        | BenchKind::Lrange500
        | BenchKind::Lrange600 => {
            let key = make_key(key_base, slot);
            let stop = (lrange_target(run.kind) - 1).to_string();
            encode_resp_parts(&[b"LRANGE", key.as_slice(), b"0", stop.as_bytes()])
        }
        BenchKind::Mset => build_mset_command(key_base, slot, value),
        BenchKind::Custom => build_custom_command(
            run.command.as_ref().expect("custom command"),
            random,
            run.random_keyspace_len,
        )?,
    })
}

fn build_custom_command(
    template: &CommandTemplate,
    random: &mut RandomSource,
    keyspace: Option<u64>,
) -> Result<Vec<u8>, String> {
    let mut parts = Vec::with_capacity(template.parts.len());
    let mut owned = Vec::with_capacity(template.parts.len());
    for part in &template.parts {
        match part {
            ArgTemplate::Literal(value) => {
                owned.push(value.clone());
            }
            ArgTemplate::RandomInt => {
                let range =
                    keyspace.ok_or_else(|| "__rand_int__ requires -r <keyspacelen>".to_string())?;
                owned.push((random.next() % range).to_string().into_bytes());
            }
        }
    }

    for item in &owned {
        parts.push(item.as_slice());
    }
    Ok(encode_resp_parts(&parts))
}

fn build_mset_command(key_base: &[u8], slot: u64, value: &[u8]) -> Vec<u8> {
    let mut owned = Vec::with_capacity(MSET_KEYS * 2);
    let mut parts = Vec::with_capacity(1 + MSET_KEYS * 2);
    parts.push(b"MSET".as_slice());
    for index in 0..MSET_KEYS {
        let key = format!("{}:{}:{}", String::from_utf8_lossy(key_base), slot, index).into_bytes();
        owned.push(key);
        owned.push(value.to_vec());
    }
    for item in &owned {
        parts.push(item.as_slice());
    }
    encode_resp_parts(&parts)
}

fn expected_response(kind: BenchKind, value: &[u8]) -> Option<ExpectedResponse> {
    match kind {
        BenchKind::PingInline | BenchKind::PingMbulk => Some(ExpectedResponse::Simple("PONG")),
        BenchKind::Set | BenchKind::Mset => Some(ExpectedResponse::Simple("OK")),
        BenchKind::Get | BenchKind::Lpop | BenchKind::Rpop => {
            Some(ExpectedResponse::Bulk(Some(value.to_vec())))
        }
        BenchKind::Incr => None,
        BenchKind::Lpush
        | BenchKind::Rpush
        | BenchKind::Sadd
        | BenchKind::Hset
        | BenchKind::Zadd => None,
        BenchKind::Spop => Some(ExpectedResponse::Bulk(Some(value.to_vec()))),
        BenchKind::ZpopMin => Some(ExpectedResponse::Array(vec![
            ExpectedResponse::Bulk(Some(value.to_vec())),
            ExpectedResponse::Bulk(Some(b"1".to_vec())),
        ])),
        BenchKind::Lrange100
        | BenchKind::Lrange300
        | BenchKind::Lrange500
        | BenchKind::Lrange600 => None,
        BenchKind::Custom => None,
    }
}

fn lrange_target(kind: BenchKind) -> usize {
    match kind {
        BenchKind::Lrange100 => 100,
        BenchKind::Lrange300 => 300,
        BenchKind::Lrange500 => 500,
        BenchKind::Lrange600 => 600,
        _ => LIST_ITEM_COUNT,
    }
}

fn make_key(base: &[u8], slot: u64) -> Vec<u8> {
    if slot == 0 {
        return base.to_vec();
    }
    let mut key = Vec::with_capacity(base.len() + 24);
    key.extend_from_slice(base);
    key.push(b':');
    key.extend_from_slice(slot.to_string().as_bytes());
    key
}

fn build_cumulative_distribution(samples: &[u64]) -> Vec<CumulativeBucket> {
    if samples.is_empty() {
        return Vec::new();
    }

    let max_ms = ns_to_ms(*samples.last().unwrap_or(&0));
    let mut buckets = Vec::new();
    let mut threshold = rounded_threshold(max_ms / 8.0).max(0.001);
    while threshold < max_ms {
        let count = samples.partition_point(|value| ns_to_ms(*value) <= threshold) as u64;
        if count > 0 {
            buckets.push(CumulativeBucket {
                percent: count as f64 * 100.0 / samples.len() as f64,
                latency_ms: threshold,
                cumulative_count: count,
            });
        }
        threshold += rounded_threshold(max_ms / 8.0).max(0.001);
    }
    buckets.push(CumulativeBucket {
        percent: 100.0,
        latency_ms: rounded_threshold(max_ms + rounded_threshold(max_ms / 8.0).max(0.001)),
        cumulative_count: samples.len() as u64,
    });
    buckets
}

fn rounded_threshold(value: f64) -> f64 {
    if value <= 0.001 {
        0.001
    } else if value < 0.01 {
        (value * 1000.0).ceil() / 1000.0
    } else {
        (value * 1000.0).ceil() / 1000.0
    }
}

fn percentile_ms(samples_ns: &[u64], percentile: f64) -> f64 {
    if samples_ns.is_empty() {
        return 0.0;
    }
    ns_to_ms(samples_ns[percentile_index(samples_ns.len(), percentile)])
}

fn percentile_index(len: usize, percentile: f64) -> usize {
    if len <= 1 || percentile <= 0.0 {
        return 0;
    }
    let rank = ((percentile / 100.0) * (len.saturating_sub(1)) as f64).round() as usize;
    rank.min(len.saturating_sub(1))
}

fn ns_to_ms(value: u64) -> f64 {
    value as f64 / 1_000_000.0
}
