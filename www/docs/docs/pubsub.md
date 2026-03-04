# Pub/Sub

BetterKV's Pub/Sub allows decoupled, real-time messaging between publishers and subscribers.

## Basic Usage

### Subscribe

```bash
# Subscribe to a channel
SUBSCRIBE chat:general

# Subscribe to multiple channels
SUBSCRIBE chat:general notifications:alice alerts

# Pattern subscribe (glob matching)
PSUBSCRIBE chat:*     # all chat channels
PSUBSCRIBE user:*:events
```

### Publish

```bash
PUBLISH chat:general "Hello, everyone!"
# (integer) 3  — number of subscribers who received it
```

### Unsubscribe

```bash
UNSUBSCRIBE chat:general
PUNSUBSCRIBE chat:*
```

## Node.js Example

```js title="subscriber.js"
import Redis from 'ioredis';

const sub = new Redis();

await sub.subscribe('notifications', 'alerts');

sub.on('message', (channel, message) => {
  console.log(`[${channel}] ${message}`);
});

sub.on('pmessage', (pattern, channel, message) => {
  console.log(`[${pattern} → ${channel}] ${message}`);
});
```

```js title="publisher.js"
import Redis from 'ioredis';

const pub = new Redis();

// Publish from anywhere in your app
setInterval(async () => {
  await pub.publish('notifications', JSON.stringify({
    type: 'new_message',
    from: 'alice',
    text: 'Hello!',
    timestamp: Date.now(),
  }));
}, 1000);
```

## Keyspace Notifications

BetterKV can publish events when keys are modified — useful for cache invalidation and event-driven workflows.

### Enable in Config

```ini title="betterkv.conf"
# Notify on: expired events (Ex), set events (Kx)
notify-keyspace-events "KEA"

# Event types:
# K = keyspace events (__keyspace@db__:key)
# E = keyevent events  (__keyevent@db__:event)
# g = generic commands (DEL, EXPIRE, RENAME)
# $ = string commands
# l = list commands
# s = set commands
# h = hash commands
# z = sorted set commands
# x = expired events
# d = stream commands
# A = all events (alias for g$lshzxd)
```

### Subscribe to Key Events

```bash
# Get notified when any key expires
SUBSCRIBE __keyevent@0__:expired

# Get notified on all events for a specific key
SUBSCRIBE __keyspace@0__:user:1
```

```js title="cache-invalidation.js"
const sub = new Redis();

await sub.psubscribe('__keyevent@0__:expired');

sub.on('pmessage', (pattern, channel, key) => {
  console.log(`Key expired: ${key}`);
  // Warm up cache for this key
  warmCache(key);
});
```

## Limitations

- Subscribers in Pub/Sub mode cannot run other commands (use a separate connection)
- Messages are not persisted — offline subscribers miss them
- For durable messaging, use [Streams](data-types#streams) instead

:::tip
For production event streaming with consumer groups, delivery guarantees, and replay, use **Streams** (`XADD`, `XREADGROUP`) instead of Pub/Sub.
:::
