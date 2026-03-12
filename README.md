# BetterKV

BetterKV is a Redis-compatible in-memory key-value store written in Rust and built for low latency, high throughput, and predictable performance.

Current status: open beta.

- Redis protocol compatible for a growing command surface
- Multi-threaded server architecture built on Tokio
- Sharded in-memory engine optimized for concurrent access
- Persistence, Pub/Sub, transactions, scripting, and more are in active development across the workspace
- Authentication and ACL support are still beta and are not recommended for production use yet

## Project status

BetterKV is not production-ready yet.

Use it today for:

- local development
- benchmarking
- compatibility testing
- exploratory integrations

Track development on GitHub: [github.com/1jmdev/BetterKV](https://github.com/1jmdev/BetterKV)

## Workspace layout

- `crates/server` - BetterKV server binary (`betterkv-server`)
- `crates/cli` - interactive CLI client (`betterkv-cli`)
- `crates/benchmark-cli` - benchmark tool (`betterkv-benchmark`)
- `crates/test-cli` - test runner utility (`betterkv-tester`)
- `crates/engine` - storage engine, sharding, expiration, and core data structures
- `crates/protocol` - RESP parsing and encoding
- `crates/commands` - Redis command execution layer
- `crates/types` - shared protocol and command types
- `crates/alloc` - memory allocator optimized for betterkv
- `crates/rehash` - hash table rehashing support

## Quick start

Build the workspace:

```bash
cargo build --release
```

Start the server:

```bash
cargo run -p betterkv-server -- --bind 127.0.0.1 --port 6379
```

Show server flags:

```bash
cargo run -p betterkv-server -- --help
```

Connect with `redis-cli`:

```bash
redis-cli -h 127.0.0.1 -p 6379 ping
# PONG
```

Run the interactive CLI:

```bash
cargo run -p betterkv-cli -- --host 127.0.0.1 --port 6379
```

Run the benchmark tool:

```bash
cargo run -p betterkv-benchmark -- -h 127.0.0.1 -p 6379 -c 100 -n 100000 -P 16
```

## Development checks

Run tests:

```bash
cargo test
```

Run Clippy across the workspace:

```bash
cargo clippy --workspace --all-targets
```

## Documentation

- Docs: [docs.betterkv.com](https://docs.betterkv.com)
- Quick start: [docs.betterkv.com/quick-start](https://docs.betterkv.com/quick-start)
- Installation: [docs.betterkv.com/installation](https://docs.betterkv.com/installation)

## Security note

BetterKV is still in beta. In particular, authentication and ACL features are not fully complete and should not be relied on for production deployments yet.

## License

Licensed under the [Elastic License 2.0](./LICENSE).

You are free to use, modify, and distribute this software. You may not provide BetterKV as a managed or hosted service to third parties without a separate license.

For a hosting or service license, contact `license@1jm.dev`.
