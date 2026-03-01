import { PageHeader } from "@/components/PageHeader";
import { DocsPager } from "@/components/DocsPager";
import { Badge } from "@/components/ui/badge";

interface CommandStatus {
  command: string;
  status: "supported" | "partial" | "planned";
  notes?: string;
}

const commandGroups: { title: string; commands: CommandStatus[] }[] = [
  {
    title: "String Commands",
    commands: [
      { command: "SET", status: "supported", notes: "Including EX, PX, NX, XX options" },
      { command: "GET", status: "supported" },
      { command: "MSET", status: "supported" },
      { command: "MGET", status: "supported" },
      { command: "INCR / INCRBY", status: "supported" },
      { command: "DECR / DECRBY", status: "supported" },
      { command: "INCRBYFLOAT", status: "supported" },
      { command: "APPEND", status: "supported" },
      { command: "STRLEN", status: "supported" },
      { command: "SETNX", status: "supported" },
      { command: "SETEX / PSETEX", status: "supported" },
      { command: "GETSET", status: "supported" },
      { command: "GETDEL", status: "supported" },
      { command: "GETRANGE", status: "supported" },
      { command: "SETRANGE", status: "supported" },
      { command: "SUBSTR", status: "planned" },
    ],
  },
  {
    title: "List Commands",
    commands: [
      { command: "LPUSH / RPUSH", status: "supported" },
      { command: "LPOP / RPOP", status: "supported" },
      { command: "LRANGE", status: "supported" },
      { command: "LLEN", status: "supported" },
      { command: "LINDEX", status: "supported" },
      { command: "LSET", status: "supported" },
      { command: "LTRIM", status: "supported" },
      { command: "LINSERT", status: "supported" },
      { command: "LREM", status: "supported" },
      { command: "BLPOP / BRPOP", status: "supported" },
      { command: "LPOS", status: "supported" },
      { command: "LMPOP", status: "planned" },
    ],
  },
  {
    title: "Hash Commands",
    commands: [
      { command: "HSET / HMSET", status: "supported" },
      { command: "HGET", status: "supported" },
      { command: "HMGET", status: "supported" },
      { command: "HGETALL", status: "supported" },
      { command: "HDEL", status: "supported" },
      { command: "HEXISTS", status: "supported" },
      { command: "HKEYS / HVALS", status: "supported" },
      { command: "HLEN", status: "supported" },
      { command: "HINCRBY", status: "supported" },
      { command: "HINCRBYFLOAT", status: "supported" },
      { command: "HSETNX", status: "supported" },
      { command: "HSCAN", status: "supported" },
      { command: "HRANDFIELD", status: "planned" },
    ],
  },
  {
    title: "Set Commands",
    commands: [
      { command: "SADD", status: "supported" },
      { command: "SREM", status: "supported" },
      { command: "SMEMBERS", status: "supported" },
      { command: "SISMEMBER", status: "supported" },
      { command: "SCARD", status: "supported" },
      { command: "SINTER / SUNION / SDIFF", status: "supported" },
      { command: "SINTERSTORE / SUNIONSTORE / SDIFFSTORE", status: "supported" },
      { command: "SRANDMEMBER", status: "supported" },
      { command: "SPOP", status: "supported" },
      { command: "SMISMEMBER", status: "supported" },
      { command: "SSCAN", status: "supported" },
    ],
  },
  {
    title: "Sorted Set Commands",
    commands: [
      { command: "ZADD", status: "supported", notes: "Including NX, XX, GT, LT options" },
      { command: "ZREM", status: "supported" },
      { command: "ZSCORE", status: "supported" },
      { command: "ZRANK / ZREVRANK", status: "supported" },
      { command: "ZRANGE / ZREVRANGE", status: "supported" },
      { command: "ZRANGEBYSCORE", status: "supported" },
      { command: "ZRANGEBYLEX", status: "supported" },
      { command: "ZCARD", status: "supported" },
      { command: "ZCOUNT", status: "supported" },
      { command: "ZINCRBY", status: "supported" },
      { command: "ZINTERSTORE / ZUNIONSTORE", status: "partial", notes: "WEIGHTS supported, AGGREGATE partial" },
      { command: "ZPOPMIN / ZPOPMAX", status: "supported" },
      { command: "ZSCAN", status: "supported" },
      { command: "ZRANDMEMBER", status: "planned" },
      { command: "ZMPOP", status: "planned" },
    ],
  },
  {
    title: "Key Commands",
    commands: [
      { command: "DEL", status: "supported" },
      { command: "EXISTS", status: "supported" },
      { command: "EXPIRE / PEXPIRE", status: "supported" },
      { command: "EXPIREAT / PEXPIREAT", status: "supported" },
      { command: "TTL / PTTL", status: "supported" },
      { command: "PERSIST", status: "supported" },
      { command: "TYPE", status: "supported" },
      { command: "RENAME / RENAMENX", status: "supported" },
      { command: "KEYS", status: "supported" },
      { command: "SCAN", status: "supported" },
      { command: "UNLINK", status: "supported" },
      { command: "OBJECT", status: "partial", notes: "ENCODING and REFCOUNT supported" },
      { command: "DUMP / RESTORE", status: "planned" },
      { command: "WAIT", status: "planned" },
    ],
  },
  {
    title: "Server Commands",
    commands: [
      { command: "PING", status: "supported" },
      { command: "ECHO", status: "supported" },
      { command: "INFO", status: "supported", notes: "server, memory, clients, stats sections" },
      { command: "DBSIZE", status: "supported" },
      { command: "FLUSHDB / FLUSHALL", status: "supported" },
      { command: "SELECT", status: "supported" },
      { command: "CONFIG GET / SET", status: "partial", notes: "Common parameters supported" },
      { command: "AUTH", status: "supported" },
      { command: "SHUTDOWN", status: "supported" },
      { command: "TIME", status: "supported" },
      { command: "COMMAND", status: "partial" },
      { command: "CLIENT", status: "partial", notes: "SETNAME, GETNAME, LIST, ID" },
      { command: "MULTI / EXEC / DISCARD", status: "supported" },
      { command: "WATCH / UNWATCH", status: "supported" },
      { command: "SUBSCRIBE / PUBLISH", status: "planned" },
      { command: "CLUSTER", status: "planned" },
      { command: "SCRIPT / EVAL", status: "planned" },
    ],
  },
];

function StatusBadge({ status }: { status: CommandStatus["status"] }) {
  if (status === "supported") {
    return (
      <Badge variant="outline" className="border-green-500/20 text-green-400 text-xs">
        Supported
      </Badge>
    );
  }
  if (status === "partial") {
    return (
      <Badge variant="outline" className="border-yellow-500/20 text-yellow-400 text-xs">
        Partial
      </Badge>
    );
  }
  return (
    <Badge variant="outline" className="border-muted-foreground/20 text-muted-foreground text-xs">
      Planned
    </Badge>
  );
}

export function DocsCompatibility() {
  return (
    <div>
      <PageHeader
        title="Redis Compatibility"
        description="JustKV implements the Redis RESP protocol. This page tracks which commands are supported."
      />

      <div className="prose-docs mt-8 space-y-8">
        <section>
          <h2>Protocol Compatibility</h2>
          <p>
            JustKV implements RESP2 (REdis Serialization Protocol version 2),
            which is the same protocol used by Redis 2.x through 6.x. This
            means any Redis client library that uses RESP2 will work with
            JustKV without modifications.
          </p>
          <p>
            RESP3 support is on the roadmap. In the meantime, most client
            libraries default to RESP2 anyway, so this is not a practical
            limitation for most use cases.
          </p>
        </section>

        <section>
          <h2>Status Legend</h2>
          <div className="not-prose flex gap-4 flex-wrap">
            <div className="flex items-center gap-2">
              <StatusBadge status="supported" />
              <span className="text-sm text-muted-foreground">
                Fully implemented
              </span>
            </div>
            <div className="flex items-center gap-2">
              <StatusBadge status="partial" />
              <span className="text-sm text-muted-foreground">
                Partially implemented
              </span>
            </div>
            <div className="flex items-center gap-2">
              <StatusBadge status="planned" />
              <span className="text-sm text-muted-foreground">
                Not yet implemented
              </span>
            </div>
          </div>
        </section>

        {commandGroups.map((group) => (
          <section key={group.title}>
            <h2>{group.title}</h2>
            <div className="not-prose overflow-hidden rounded-lg border border-border/50">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-border/50 bg-card/40">
                    <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                      Command
                    </th>
                    <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                      Status
                    </th>
                    <th className="px-4 py-2 text-left font-medium text-muted-foreground hidden sm:table-cell">
                      Notes
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {group.commands.map((cmd) => (
                    <tr
                      key={cmd.command}
                      className="border-b border-border/50 last:border-b-0"
                    >
                      <td className="px-4 py-2">
                        <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                          {cmd.command}
                        </code>
                      </td>
                      <td className="px-4 py-2">
                        <StatusBadge status={cmd.status} />
                      </td>
                      <td className="px-4 py-2 text-muted-foreground hidden sm:table-cell">
                        {cmd.notes || "—"}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        ))}

        <section>
          <h2>Known Differences</h2>
          <ul>
            <li>
              <strong>Multi-threading:</strong> JustKV processes commands across
              multiple threads. While individual commands are atomic, the
              ordering of concurrent commands from different clients may differ
              from single-threaded Redis.
            </li>
            <li>
              <strong>Lua scripting:</strong> Not yet supported. Use
              transactions (MULTI/EXEC) for atomic operations.
            </li>
            <li>
              <strong>Pub/Sub:</strong> Not yet implemented. Planned for a
              future release.
            </li>
            <li>
              <strong>Persistence:</strong> RDB-style snapshotting is
              experimental. AOF is planned. JustKV is currently best suited for
              caching and ephemeral data.
            </li>
            <li>
              <strong>Clustering:</strong> Not yet supported. Each JustKV
              instance is standalone. Client-side sharding works with any
              compatible client.
            </li>
          </ul>
        </section>

        <section>
          <h2>Migration from Redis</h2>
          <p>
            For most caching and session storage workloads, migrating from Redis
            to JustKV is straightforward:
          </p>
          <ol>
            <li>Install JustKV and start the server on the same port</li>
            <li>
              Point your application's Redis connection to the JustKV instance
            </li>
            <li>
              Verify your application's commands are in the supported list above
            </li>
            <li>
              If you use Lua scripts, refactor them to use MULTI/EXEC
              transactions
            </li>
          </ol>
          <p>
            If you find a supported command that doesn't behave correctly,
            please{" "}
            <a
              href="https://github.com/1jmdev/justkv/issues"
              target="_blank"
              rel="noopener noreferrer"
              className="text-primary hover:underline"
            >
              open an issue
            </a>
            .
          </p>
        </section>
      </div>

      <DocsPager currentHref="/docs/compatibility" />
    </div>
  );
}
