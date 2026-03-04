# Replication

BetterKV supports primary-replica replication out of the box. Replicas receive a full copy of data and stream all subsequent writes from the primary.

## How It Works

```
┌─────────────┐      replication stream      ┌──────────────┐
│   Primary   │ ────────────────────────────▶ │  Replica 1   │
│  (read/write)│                              │  (read-only) │
└─────────────┘                              └──────────────┘
       │                                      ┌──────────────┐
       └─────────────────────────────────────▶ │  Replica 2   │
                                              │  (read-only) │
                                              └──────────────┘
```

1. Replica connects to primary and sends `PSYNC`
2. Primary sends an RDB snapshot (full sync on first connect)
3. Replica loads the snapshot and starts receiving commands
4. Subsequent writes stream via the replication backlog

## Configuration

### On the Primary

```ini title="betterkv-primary.conf"
port 6379
requirepass primary_secret

# Replication backlog (for reconnect without full sync)
repl-backlog-size 64mb
repl-backlog-ttl 3600

# Require N replicas before accepting writes (for safety)
# min-replicas-to-write 1
# min-replicas-max-lag 10
```

### On Each Replica

```ini title="betterkv-replica.conf"
port 6380
replicaof 192.168.1.10 6379
masterauth primary_secret

# Replicas are read-only by default
replica-read-only yes

# Don't serve stale data if replication link is broken
replica-serve-stale-data no

# Priority for Sentinel failover (lower = preferred)
replica-priority 100
```

## Checking Replication Status

```bash
# On primary
betterkv-cli INFO replication
# role:master
# connected_slaves:2
# slave0:ip=192.168.1.11,port=6380,state=online,offset=12345
# slave1:ip=192.168.1.12,port=6380,state=online,offset=12340
# master_replid:abc123...
# master_repl_offset:12345

# Check replication lag
betterkv-cli INFO replication | grep lag
```

## Read Scaling

Route read queries to replicas to reduce load on the primary:

```js title="app.js — Node.js with ioredis"
import Redis from 'ioredis';

const primary = new Redis({ host: 'primary', port: 6379 });
const replica = new Redis({ host: 'replica1', port: 6380 });

// Writes always go to primary
await primary.set('key', 'value');

// Reads can go to replica
const val = await replica.get('key');
```

## Sentinel (Automatic Failover)

Sentinel monitors your primary and automatically promotes a replica if the primary fails.

```ini title="sentinel.conf"
sentinel monitor bkv-primary 192.168.1.10 6379 2
sentinel auth-pass bkv-primary primary_secret
sentinel down-after-milliseconds bkv-primary 5000
sentinel failover-timeout bkv-primary 60000
sentinel parallel-syncs bkv-primary 1
```

Start Sentinel:

```bash
betterkv-sentinel /etc/betterkv/sentinel.conf
# or
betterkv-server /etc/betterkv/sentinel.conf --sentinel
```

:::tip
Run at least 3 Sentinel instances on separate hosts to avoid split-brain. A quorum of 2 (`2` in the monitor line) is standard.
:::
