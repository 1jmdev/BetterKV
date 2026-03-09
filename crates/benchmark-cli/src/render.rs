use crate::args::Args;
use crate::bench::BenchResult;

pub fn render_result(args: &Args, result: &BenchResult) {
    if args.csv {
        println!(
            "{},{},{},{},{:.6},{:.2},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{},{},{},{}",
            result.name,
            result.requests,
            result.warmup_requests,
            result.clients,
            result.elapsed_secs,
            result.req_per_sec,
            result.avg_ms,
            result.min_ms,
            result.p50_ms,
            result.p95_ms,
            result.p99_ms,
            result.max_ms,
            result.pipeline,
            result.data_size,
            result.random_keys,
            result.keyspace,
        );
        return;
    }

    if args.quiet {
        println!(
            "{}: {:.2} requests per second",
            result.name, result.req_per_sec
        );
        return;
    }

    println!("====== {} ======", result.name);
    println!(
        "  {} requests completed in {:.2} seconds",
        result.requests, result.elapsed_secs
    );
    if result.warmup_requests > 0 {
        println!("  {} requests completed as warmup", result.warmup_requests);
    }
    println!("  {} parallel clients", result.clients);
    println!("  {} bytes payload", result.data_size);
    println!("  keep alive: 1");
    println!(
        "  multi-thread: {}",
        if args.threads > 1 { "yes" } else { "no" }
    );
    println!("  {} pipeline depth", result.pipeline);
    println!(
        "  random keys: {} (keyspace {})",
        result.random_keys, result.keyspace
    );
    println!(
        "  validation: {}",
        if args.strict {
            "strict (check every response)"
        } else {
            "none (throughput only)"
        }
    );
    print_latency_distributions(result);
    println!("\nSummary:");
    println!(
        "  throughput summary: {:.2} requests per second",
        result.req_per_sec
    );
    println!("  latency summary (msec):");
    println!("          avg       min       p50       p95       p99       max");
    println!(
        "      {:7.3}   {:7.3}   {:7.3}   {:7.3}   {:7.3}   {:7.3}",
        result.avg_ms, result.min_ms, result.p50_ms, result.p95_ms, result.p99_ms, result.max_ms
    );
    println!();
}

fn print_latency_distributions(result: &BenchResult) {
    if result.samples_ns.is_empty() {
        return;
    }

    const PCTS: &[f64] = &[
        0.0, 50.0, 75.0, 87.5, 93.75, 96.875, 98.438, 99.219, 99.609, 99.805, 99.902, 99.951,
        99.976, 99.988, 99.994, 99.997, 99.999, 100.0,
    ];
    const CDF_BINS_MS: &[f64] = &[0.1, 0.2, 0.3, 0.4, 0.5];

    println!("\nLatency by percentile distribution:");
    for pct in PCTS {
        let (ms, count) = percentile_value_and_count(&result.samples_ns, *pct);
        println!("{pct:.3}% <= {ms:.3} milliseconds (cumulative count {count})");
    }

    println!("\nCumulative distribution of latencies:");
    for max_ms in CDF_BINS_MS {
        let count = result
            .samples_ns
            .partition_point(|ns| *ns as f64 <= *max_ms * 1_000_000.0);
        let pct = count as f64 * 100.0 / result.samples_ns.len() as f64;
        println!("{pct:.3}% <= {max_ms:.3} milliseconds (cumulative count {count})");
    }
}

fn percentile_value_and_count(sorted_ns: &[u64], pct: f64) -> (f64, usize) {
    let n = sorted_ns.len();
    if n == 0 {
        return (0.0, 0);
    }
    let idx = if pct <= 0.0 {
        0
    } else if pct >= 100.0 {
        n - 1
    } else {
        ((pct / 100.0) * (n as f64 - 1.0)).round() as usize
    };
    (sorted_ns[idx] as f64 / 1_000_000.0, idx + 1)
}
