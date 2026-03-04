# Server Commands

Commands for server management, monitoring, and administration.

## INFO

```
INFO [section]
```

Returns server statistics and status. Sections: `server`, `clients`, `memory`, `persistence`, `stats`, `replication`, `cpu`, `commandstats`, `latencystats`, `cluster`, `keyspace`, `all`, `everything`.

**Complexity:** O(1)

```bash
INFO                    # all standard sections
INFO memory             # memory stats only
INFO replication        # replication status
INFO keyspace           # key count per DB

# Key fields:
# used_memory_human: 2.50M
# connected_clients: 42
# instantaneous_ops_per_sec: 15234
# uptime_in_seconds: 86400
# role: master
# rdb_last_save_time: 1709500000
```

## CONFIG GET / SET / REWRITE / RESETSTAT

```
CONFIG GET parameter [parameter ...]
CONFIG SET parameter value [parameter value ...]
CONFIG REWRITE
CONFIG RESETSTAT
```

Read or modify server configuration at runtime.

**Complexity:** O(N) for GET with glob, O(1) for specific parameter

```bash
# Get current values
CONFIG GET maxmemory
CONFIG GET save
CONFIG GET *max*       # all params matching *max*

# Set values
CONFIG SET maxmemory 8gb
CONFIG SET loglevel verbose
CONFIG SET save "3600 1 300 100 60 10000"

# Persist changes back to config file
CONFIG REWRITE

# Reset stats counters
CONFIG RESETSTAT
```

## DBSIZE

```
DBSIZE
```

Returns the number of keys in the current database.

**Complexity:** O(1)

```bash
DBSIZE    # 1542893
```

## SELECT

```
SELECT index
```

Switch to database `index` (0–15 by default).

**Complexity:** O(1)

```bash
SELECT 0   # default DB
SELECT 1   # switch to DB 1
SET foo "in-db-1"
SELECT 0
GET foo    # (nil) — different DB
```

:::info
Cluster mode only supports database 0. Multi-DB is a standalone-only feature.
:::

## FLUSHDB / FLUSHALL

```
FLUSHDB [ASYNC | SYNC]
FLUSHALL [ASYNC | SYNC]
```

Delete all keys in the current DB or all DBs. Use `ASYNC` for non-blocking.

```bash
FLUSHDB ASYNC     # delete current DB in background
FLUSHALL ASYNC    # delete ALL databases in background
```

:::danger
These commands are irreversible. Rename or disable them in production:
```ini
rename-command FLUSHDB ""
rename-command FLUSHALL ""
```
:::

## PING

```
PING [message]
```

Test connectivity. Returns `PONG` or echoes the message.

```bash
PING              # PONG
PING "are you there?"   # "are you there?"
```

## ECHO

```
ECHO message
```

Returns the message. Useful for debugging.

```bash
ECHO "hello"    # "hello"
```

## COMMAND

```
COMMAND [COUNT | DOCS [command-name ...] | GETKEYS command | INFO [command-name ...] | LIST [FILTERBY ...]]
```

Introspect available commands.

```bash
COMMAND COUNT              # number of commands
COMMAND INFO GET SET DEL   # details about specific commands
COMMAND DOCS GET           # full documentation for GET
COMMAND GETKEYS MSET k1 v1 k2 v2  # which args are keys
```

## CLIENT

```
CLIENT ID
CLIENT GETNAME
CLIENT SETNAME name
CLIENT LIST [TYPE normal|master|replica|pubsub] [ID id ...]
CLIENT KILL [ID id] [ADDR addr] [LADDR addr]
CLIENT PAUSE timeout [WRITE | ALL]
CLIENT UNPAUSE
CLIENT NO-EVICT on|off
CLIENT INFO
```

Manage client connections.

```bash
CLIENT ID               # current connection ID
CLIENT SETNAME "my-app" # label this connection
CLIENT LIST             # all connected clients
CLIENT LIST TYPE normal  # only normal clients
CLIENT KILL ID 42       # disconnect client 42
```

## LATENCY

```
LATENCY LATEST
LATENCY HISTORY event-name
LATENCY RESET [event-name ...]
```

Monitor latency spikes.

```bash
LATENCY LATEST          # all recent latency events
LATENCY HISTORY command # history for a specific event
LATENCY RESET           # clear all latency data
```

## MEMORY

```
MEMORY USAGE key [SAMPLES count]
MEMORY DOCTOR
MEMORY STATS
MEMORY MALLOC-STATS
MEMORY PURGE
```

Inspect memory usage.

```bash
MEMORY USAGE user:1       # bytes used by this key
MEMORY DOCTOR             # diagnostic suggestions
MEMORY STATS              # detailed memory breakdown
MEMORY PURGE              # force jemalloc to release memory
```

## SAVE / BGSAVE / BGREWRITEAOF

```
SAVE
BGSAVE [SCHEDULE]
BGREWRITEAOF
LASTSAVE
```

Manage persistence manually.

```bash
BGSAVE              # trigger background RDB save
BGREWRITEAOF        # trigger background AOF rewrite
LASTSAVE            # unix timestamp of last successful save
SAVE                # synchronous save (BLOCKS — avoid in production)
```

## SHUTDOWN

```
SHUTDOWN [NOSAVE | SAVE] [NOW] [FORCE] [ABORT]
```

Shutdown the server.

```bash
SHUTDOWN SAVE       # save then shutdown
SHUTDOWN NOSAVE     # shutdown without saving
```

:::danger
`SHUTDOWN NOSAVE` discards all data since the last snapshot. Only use when intentional.
:::
