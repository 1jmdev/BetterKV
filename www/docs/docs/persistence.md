# Persistence

BetterKV offers two persistence mechanisms: **RDB** (point-in-time snapshots) and **AOF** (append-only log). You can use either or both.

## RDB — Snapshot Persistence

RDB produces compact, point-in-time snapshots of your dataset using a fork-based background save.

**Pros:** Compact file, fast restarts, minimal runtime overhead.  
**Cons:** Data since last snapshot is lost on crash.

### Configuration

```ini title="betterkv.conf"
# Trigger a save when N keys change within M seconds
save 3600 1       # 1+ keys changed in 1 hour
save 300 100      # 100+ keys changed in 5 minutes
save 60 10000     # 10000+ keys changed in 1 minute

# Snapshot filename and directory
dbfilename dump.rdb
dir /var/lib/betterkv

# Compression (recommended)
rdbcompression yes

# Checksum (integrity check on load)
rdbchecksum yes
```

### Manual Snapshot

```bash
# Synchronous (blocks until done — avoid in production)
betterkv-cli BGSAVE SYNC

# Background (non-blocking)
betterkv-cli BGSAVE

# Check last save time
betterkv-cli LASTSAVE
```

## AOF — Append-Only File

AOF logs every write command. On restart, BetterKV replays the log to reconstruct the dataset.

**Pros:** Much more durable — can lose at most 1 second of data.  
**Cons:** Larger files, slightly slower writes.

### Configuration

```ini title="betterkv.conf"
appendonly yes
appendfilename "appendonly.aof"
appenddirname "appendonlydir"

# fsync policy
appendfsync everysec   # recommended: sync every second
# appendfsync always   # safest: sync after every write
# appendfsync no       # fastest: let OS decide

# Prevent fsync during rewrites (reduces latency spikes)
no-appendfsync-on-rewrite yes

# Auto-rewrite when AOF is N% larger than last rewrite
auto-aof-rewrite-percentage 100
auto-aof-rewrite-min-size 64mb
```

### Manual AOF Rewrite

```bash
# Trigger background rewrite (compacts the log)
betterkv-cli BGREWRITEAOF
```

## Using Both (Recommended for Production)

```ini title="betterkv.conf"
# RDB: safety net and fast restart
save 3600 1
save 300 100
save 60 10000

# AOF: durability
appendonly yes
appendfsync everysec
no-appendfsync-on-rewrite yes
auto-aof-rewrite-percentage 100
auto-aof-rewrite-min-size 64mb
```

When both are enabled:
- BetterKV prefers AOF on restart (more complete data)
- RDB is still created and can be used for backups

## Durability Comparison

| Mode             | Max Data Loss | Write Latency | File Size |
|------------------|---------------|---------------|-----------|
| No persistence   | All data      | Minimal       | None      |
| RDB only         | Minutes       | Low           | Small     |
| AOF (everysec)   | ~1 second     | Low           | Large     |
| AOF (always)     | ~1 command    | High          | Large     |
| RDB + AOF        | ~1 second     | Low           | Both      |

## Backup Strategy

```bash
#!/bin/bash
# backup.sh — copy snapshot to S3

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
betterkv-cli BGSAVE

# Wait for save to complete
while [ $(betterkv-cli LASTSAVE) -eq $LAST_SAVE ]; do
  sleep 1
done

cp /var/lib/betterkv/dump.rdb /backups/dump_${TIMESTAMP}.rdb
aws s3 cp /backups/dump_${TIMESTAMP}.rdb s3://my-bucket/backups/
```

:::tip
For production, combine AOF (durability) with periodic RDB copies to object storage (recovery). Keep at least 7 days of RDB snapshots.
:::
