use crate::args::Args;
use crate::bench::BenchResult;

pub fn render_result(args: &Args, result: &BenchResult) {
    if args.csv {
        println!(
            "{},{},{},{},{:.6},{:.2},{:.4},{:.4},{:.4},{:.4},{},{},{},{}",
            result.name,
            result.scenario.unwrap_or(""),
            result.requests,
            result.clients,
            result.elapsed_secs,
            result.req_per_sec,
            result.avg_ms,
            result.p50_ms,
            result.p95_ms,
            result.p99_ms,
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
    if let Some(scenario) = result.scenario {
        println!("  scenario: {}", scenario);
    }
    println!(
        "  {} requests completed in {:.2} seconds",
        result.requests, result.elapsed_secs
    );
    println!("  {} parallel clients", result.clients);
    println!("  {} bytes payload", result.data_size);
    println!("  {} pipeline depth", result.pipeline);
    println!(
        "  random keys: {} (keyspace {})",
        result.random_keys, result.keyspace
    );
    println!(
        "  latency avg/p50/p95/p99 = {:.4}/{:.4}/{:.4}/{:.4} ms",
        result.avg_ms, result.p50_ms, result.p95_ms, result.p99_ms
    );
    println!("  {:.2} requests per second", result.req_per_sec);
    println!();
}
