# Configuration

BetterKV is configured via a config file (`betterkv.conf`) or command-line flags. All Redis `redis.conf` options are supported.

## Starting with a Config File

```bash
betterkv-server /etc/betterkv/betterkv.conf
```

Override individual options via CLI:

```bash
betterkv-server /etc/betterkv/betterkv.conf --port 6380 --loglevel verbose
```

## Essential Options

### Network

```ini title="betterkv.conf"
# Bind address (default: all interfaces)
bind 127.0.0.1 -::1

# Port (default: 6379)
port 6379

# Unix socket (for local-only, lower latency)
# unixsocket /tmp/betterkv.sock
# unixsocketperm 700

# Max connected clients
maxclients 10000

# TCP keepalive
tcp-keepalive 300
```

### Memory

```ini title="betterkv.conf"
# Maximum memory limit (0 = no limit)
maxmemory 4gb

# Eviction policy when maxmemory is reached
# Options: noeviction, allkeys-lru, volatile-lru,
#          allkeys-lfu, volatile-lfu, allkeys-random,
#          volatile-random, volatile-ttl
maxmemory-policy allkeys-lru

# LRU / LFU sampling size (higher = more accurate, more CPU)
maxmemory-samples 10
```

### Persistence — RDB Snapshots

```ini title="betterkv.conf"
# Save snapshot if N keys changed in M seconds
save 3600 1      # 1 key changed in 1 hour
save 300 100     # 100 keys changed in 5 minutes
save 60 10000    # 10000 keys changed in 1 minute

# Disable RDB saves
# save ""

dbfilename dump.rdb
dir /var/lib/betterkv
```

### Persistence — AOF (Append-Only File)

```ini title="betterkv.conf"
appendonly yes
appendfilename "appendonly.aof"

# fsync policy
# always  - safest, slowest
# everysec - good balance (default)
# no      - fastest, OS decides
appendfsync everysec

# Auto-rewrite AOF when it grows by this %
auto-aof-rewrite-percentage 100
auto-aof-rewrite-min-size 64mb
```

### Security

```ini title="betterkv.conf"
# Set a password
requirepass your_strong_password_here

# ACL file for per-user permissions
aclfile /etc/betterkv/users.acl

# Disable dangerous commands
rename-command FLUSHDB ""
rename-command FLUSHALL ""
rename-command DEBUG ""
```

### Replication

```ini title="betterkv.conf"
# On a replica, point to primary
replicaof 192.168.1.10 6379

# Replica auth (if primary requires password)
masterauth your_primary_password

# Read-only replicas (recommended)
replica-read-only yes
```

### Performance Tuning

```ini title="betterkv.conf"
# Hash max entries before converting to full hash
hash-max-listpack-entries 128
hash-max-listpack-value 64

# List compression
list-max-listpack-size -2  # -2 = 8kb per node

# Set optimization
set-max-intset-entries 512
set-max-listpack-entries 128

# Sorted set
zset-max-listpack-entries 128
zset-max-listpack-value 64

# Lazy free (non-blocking deletes for large keys)
lazyfree-lazy-eviction yes
lazyfree-lazy-expire yes
lazyfree-lazy-server-del yes
replica-lazy-flush yes
```

## Configuration via ACL File

For fine-grained user access control:

```ini title="/etc/betterkv/users.acl"
# Syntax: user <name> [on|off] [>password] [~keypattern] [&channel] [+command|-command]

# Default user (disable completely)
user default off

# Admin
user admin on >admin_secret ~* &* +@all

# Application user — limited key space and commands
user app on >app_secret \
  ~session:* ~cache:* ~queue:* \
  +GET +SET +DEL +EXPIRE +TTL \
  +RPUSH +LPOP +LLEN \
  +HSET +HGET +HGETALL +HDEL

# Read-only monitoring user
user monitor on >monitor_secret ~* +INFO +DBSIZE +KEYS
```

Load at runtime without restart:

```bash
betterkv-cli ACL LOAD
```

## Runtime Config Changes

Most options can be changed at runtime:

```bash
# View current value
betterkv-cli CONFIG GET maxmemory

# Change value
betterkv-cli CONFIG SET maxmemory 8gb

# Persist runtime changes back to config file
betterkv-cli CONFIG REWRITE
```

:::info
`CONFIG REWRITE` requires the server was started with a config file path.
:::
