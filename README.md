# BetterKV

`BetterKV` is a lightweight Redis-compatible in-memory server written in Rust.

## Features

- RESP2 protocol parsing and encoding.
- Multi-threaded Tokio TCP server.
- Sharded in-memory key/value store for concurrent access.
- Background key expiration sweeper.
- Redis-compatible for common commands

## Workspace

- `crates/cli`: CLI binary package exposing the `betterkv-cli` and `betterkv` executable.
- `crates/betterkv-server`: Redis-style server binary exposing the `betterkv-server` executable.
- `crates/benchmark-cli`: Redis-benchmark style binary exposing the `betterkv-benchmark` executable.

## Run

```bash
cargo run -p betterkv-server -- --bind 127.0.0.1 --port 6379

# show compatible startup flags
cargo run -p betterkv-server -- --help

cargo run -p betterkv-benchmark -- -h 127.0.0.1 -p 6379 -c 100 -n 100000 -P 16
```

## License

Licensed under the [Elastic License 2.0](./LICENSE).

You're free to use, modify, and distribute this software.
The only restriction is you cannot provide it as a managed/hosted
service to others.

For a hosting/service license, contact: license@1jm.dev