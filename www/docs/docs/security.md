# Security

BetterKV is designed for trusted environments. Follow these guidelines to secure your deployment.

## Network Hardening

### Bind to Localhost or Private Network

```ini title="betterkv.conf"
# Only listen on loopback + private network
bind 127.0.0.1 10.0.0.5

# Never bind to 0.0.0.0 in production
```

### Firewall Rules

```bash
# Allow only app servers to reach BetterKV
ufw allow from 10.0.0.0/24 to any port 6379
ufw deny 6379
```

### TLS Encryption

```ini title="betterkv.conf"
# Enable TLS
tls-port 6380
port 0   # disable plaintext

tls-cert-file /etc/betterkv/tls/server.crt
tls-key-file  /etc/betterkv/tls/server.key
tls-ca-cert-file /etc/betterkv/tls/ca.crt

# Require client certificates (mTLS)
tls-auth-clients yes

# TLS version restrictions
tls-protocols "TLSv1.2 TLSv1.3"
tls-ciphers "ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-CHACHA20-POLY1305"
```

Generate a self-signed certificate for dev:

```bash
openssl req -x509 -newkey rsa:4096 \
  -keyout server.key -out server.crt \
  -days 365 -nodes -subj '/CN=betterkv'
```

## Authentication

### Password Auth (Simple)

```ini title="betterkv.conf"
requirepass your_strong_password_here
```

```bash
betterkv-cli -a your_strong_password_here ping
# or
betterkv-cli
AUTH your_strong_password_here
```

### ACL — Per-User Permissions

ACLs give fine-grained control over which commands and keys each user can access.

```bash
# View current users
ACL LIST
ACL WHOAMI

# Create users
ACL SETUSER alice on >alice_pass ~user:* +@read +@write
ACL SETUSER readonly on >readonly_pass ~* +@read
ACL SETUSER admin on >admin_pass ~* &* +@all
```

ACL file (for persistence across restarts):

```ini title="/etc/betterkv/users.acl"
# Syntax: user <name> [on|off] [>password] [~keypattern] [&channel] [+command|-command]

# Default user (disable completely)
user default off

# Admin
user admin on >admin_password ~* &* +@all

# Application user — limited key space and commands
user app on >app_password \
  ~session:* ~cache:* ~queue:* \
  +GET +SET +DEL +EXPIRE +TTL \
  +RPUSH +LPOP +LLEN \
  +HSET +HGET +HGETALL +HDEL

# Read-only monitoring user
user monitor on >monitor_secret ~* +INFO +DBSIZE +KEYS
```

Load ACL file:

```ini title="betterkv.conf"
aclfile /etc/betterkv/users.acl
```

Reload at runtime:

```bash
betterkv-cli ACL LOAD
```

## Disable Dangerous Commands

```ini title="betterkv.conf"
# Rename to empty string to disable
rename-command FLUSHDB   ""
rename-command FLUSHALL  ""
rename-command DEBUG     ""
rename-command CONFIG    ""
rename-command SHUTDOWN  ""
rename-command SLAVEOF   ""

# Or rename to a secret name
rename-command CONFIG "CONFIG_c8d3f5a2b1"
```

## Protected Mode

By default, if no password is set and BetterKV is not bound to loopback, **protected mode** blocks external connections:

```ini title="betterkv.conf"
# On (default) — refuse external connections without auth
protected-mode yes
```

## Security Checklist

- [ ] Bind to private IP, not `0.0.0.0`
- [ ] Set a strong `requirepass` or use ACLs
- [ ] Disable the `default` user in ACL
- [ ] Firewall port 6379 from the public internet
- [ ] Enable TLS in production
- [ ] Disable `FLUSHALL`, `DEBUG`, `CONFIG`
- [ ] Run BetterKV as a non-root user
- [ ] Set `protected-mode yes`
- [ ] Monitor `AUTH` failures via keyspace notifications or logs

:::danger
**Never expose BetterKV directly to the internet.** Hundreds of thousands of Redis/Valkey instances are compromised each year due to misconfiguration.
:::
