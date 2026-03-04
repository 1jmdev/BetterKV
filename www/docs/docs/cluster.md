# Cluster Mode

BetterKV Cluster provides horizontal scaling by automatically sharding data across multiple nodes.

## Concepts

- **Sharding**: Data is split into **16384 hash slots**. Each node owns a subset.
- **Slot assignment**: A key's slot is `CRC16(key) % 16384`
- **Replication**: Each primary has N replicas for high availability
- **Failover**: If a primary fails, its replica auto-promotes

## Minimum Cluster Setup

A production cluster needs at least **6 nodes** (3 primaries + 3 replicas):

```
Primary 1 (slots 0–5460)     → Replica 1A
Primary 2 (slots 5461–10922) → Replica 2A
Primary 3 (slots 10923–16383)→ Replica 3A
```

## Setting Up a Cluster

### 1. Configure Each Node

```ini title="node-7001.conf"
port 7001
cluster-enabled yes
cluster-config-file nodes-7001.conf
cluster-node-timeout 5000
appendonly yes
dir /var/lib/betterkv/7001
```

Repeat for ports 7001–7006.

### 2. Start All Nodes

```bash
for port in 7001 7002 7003 7004 7005 7006; do
  betterkv-server /etc/betterkv/node-${port}.conf &
done
```

### 3. Create the Cluster

```bash
betterkv-cli --cluster create \
  127.0.0.1:7001 \
  127.0.0.1:7002 \
  127.0.0.1:7003 \
  127.0.0.1:7004 \
  127.0.0.1:7005 \
  127.0.0.1:7006 \
  --cluster-replicas 1
```

Confirm with `yes` when prompted.

### 4. Verify

```bash
betterkv-cli --cluster info 127.0.0.1:7001
betterkv-cli --cluster check 127.0.0.1:7001
```

## Using the Cluster

Connect with cluster mode enabled in your client:

```js title="Node.js"
import Redis from 'ioredis';

const cluster = new Redis.Cluster([
  { host: '127.0.0.1', port: 7001 },
  { host: '127.0.0.1', port: 7002 },
  { host: '127.0.0.1', port: 7003 },
]);

await cluster.set('foo', 'bar');
const val = await cluster.get('foo');
```

## Hash Tags — Multi-Key Operations

To ensure multiple keys land on the same slot (required for multi-key commands), use **hash tags** `{}`:

```bash
# These all go to the same slot (based on "user:1")
SET {user:1}.name "Alice"
SET {user:1}.email "alice@example.com"
HSET {user:1}.profile age 30

# Now MGET works across keys in cluster
MGET {user:1}.name {user:1}.email
```

## Scaling — Adding Nodes

```bash
# Add a new primary
betterkv-cli --cluster add-node \
  127.0.0.1:7007 \
  127.0.0.1:7001

# Add a replica to an existing primary
betterkv-cli --cluster add-node \
  127.0.0.1:7008 \
  127.0.0.1:7001 \
  --cluster-slave \
  --cluster-master-id <primary-node-id>

# Rebalance slots evenly
betterkv-cli --cluster rebalance 127.0.0.1:7001
```

## Cluster Configuration Options

```ini title="betterkv.conf"
# Required for cluster mode
cluster-enabled yes

# Auto-generated topology file
cluster-config-file nodes.conf

# Time before a node is considered failed (ms)
cluster-node-timeout 15000

# Require all slots covered to accept writes
cluster-require-full-coverage yes

# Prefer local reads (reduce cross-slot latency)
cluster-allow-reads-when-down no

# Migration throttle (slots/second)
cluster-migration-barrier 1
```

:::warning
Multi-key commands (`MGET`, `MSET`, `DEL`, transactions with `MULTI`) require all keys to be in the same slot. Use hash tags or redesign your key schema.
:::
