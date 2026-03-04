# Introduction

**BetterKV** is a high-performance, Redis-compatible key-value store built in rust and fully open-source. It is a drop-in replacement for Redis with significant performance improvements, no licensing surprises, and a first-class developer experience.

## Why BetterKV?

Redis made a license change in 2024 that left many teams scrambling. BetterKV is Elastic License 2.0. You get:

- **10x faster** EXPIRE operations
- **4x faster** Lua script execution
- Near full Redis API compatibility
- No proprietary modules or feature gates
- Active open-source community

## Architecture Overview

BetterKV is built on a single-threaded event loop model with async I/O, giving it exceptional concurrency without the overhead of multi-threading for command processing.

```
┌───────────────────────────────────────────┐
│               BetterKV Server             │
│                                           │
│  ┌──────────┐   ┌──────────┐             │
│  │  Network │   │  Command  │             │
│  │  Layer   │──▶│ Processor │             │
│  └──────────┘   └────┬─────┘             │
│                       │                   │
│              ┌────────▼────────┐          │
│              │   Storage Engine │          │
│              │  (RDB + AOF)    │          │
│              └─────────────────┘          │
└───────────────────────────────────────────┘
```

## Feature Comparison

| Feature             | BetterKV | Redis OSS | Redis Enterprise |
|---------------------|----------|-----------|------------------|
| License             | Apache 2.0 | SSPL     | Proprietary      |
| EXPIRE performance  | 3x faster | Baseline | Baseline         |
| Lua scripting       | 9x faster | Baseline | Baseline         |
| Clustering          | Built-in  | Built-in  | Built-in         |
| Modules             | Community | Limited   | Proprietary      |
| Price               | Free      | Free      | Paid             |

## Next Steps

- [Quick Start →](quick-start) — Running BetterKV in 60 seconds
- [Installation →](installation) — All install methods
- [Data Types →](data-types) — Understand what you can store
