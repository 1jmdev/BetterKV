import { PageHeader } from "@/components/PageHeader";
import { CodeBlock } from "@/components/CodeBlock";
import { DocsPager } from "@/components/DocsPager";

export function DocsQuickstart() {
  return (
    <div>
      <PageHeader
        title="Quickstart"
        description="Get JustKV running and store your first key-value pair in under a minute."
      />

      <div className="prose-docs mt-8 space-y-8">
        <section>
          <h2>1. Start the Server</h2>
          <p>
            After{" "}
            <a href="/docs/installation" className="text-primary hover:underline">
              installing
            </a>
            , start the JustKV server with default settings:
          </p>
          <CodeBlock language="bash" code="justkv-server" />
          <p>
            By default, JustKV listens on <code>127.0.0.1:6379</code> and uses
            all available CPU cores. You'll see output like:
          </p>
          <CodeBlock
            language="plain"
            code={`[2026-02-28 10:15:32] INFO  JustKV 0.1.0 starting
[2026-02-28 10:15:32] INFO  Binding to 127.0.0.1:6379
[2026-02-28 10:15:32] INFO  Worker threads: 8
[2026-02-28 10:15:32] INFO  Max memory: unlimited
[2026-02-28 10:15:32] INFO  Ready to accept connections`}
          />
        </section>

        <section>
          <h2>2. Connect with a Client</h2>
          <p>
            Since JustKV speaks the Redis protocol, you can connect using{" "}
            <code>redis-cli</code> or any Redis client library. If you have
            Redis tools installed:
          </p>
          <CodeBlock language="bash" code="redis-cli -h 127.0.0.1 -p 6379" />
          <p>
            Alternatively, use the bundled <code>justkv-cli</code>:
          </p>
          <CodeBlock language="bash" code="justkv-cli" />
        </section>

        <section>
          <h2>3. Basic Operations</h2>
          <p>Try some basic key-value operations:</p>
          <CodeBlock
            title="Strings"
            code={`127.0.0.1:6379> SET user:1:name "Alice"
OK
127.0.0.1:6379> GET user:1:name
"Alice"
127.0.0.1:6379> SET counter 0
OK
127.0.0.1:6379> INCR counter
(integer) 1
127.0.0.1:6379> INCRBY counter 10
(integer) 11`}
          />

          <CodeBlock
            title="Hashes"
            code={`127.0.0.1:6379> HSET user:1 name "Alice" email "alice@example.com" role "admin"
(integer) 3
127.0.0.1:6379> HGET user:1 name
"Alice"
127.0.0.1:6379> HGETALL user:1
1) "name"
2) "Alice"
3) "email"
4) "alice@example.com"
5) "role"
6) "admin"`}
          />

          <CodeBlock
            title="Lists"
            code={`127.0.0.1:6379> LPUSH queue:tasks "send-email" "process-payment" "generate-report"
(integer) 3
127.0.0.1:6379> RPOP queue:tasks
"send-email"
127.0.0.1:6379> LLEN queue:tasks
(integer) 2`}
          />

          <CodeBlock
            title="Key Expiration"
            code={`127.0.0.1:6379> SET session:abc123 "user:1" EX 3600
OK
127.0.0.1:6379> TTL session:abc123
(integer) 3599`}
          />
        </section>

        <section>
          <h2>4. Using with Application Code</h2>
          <p>
            Use any Redis client library. Here are examples in common languages:
          </p>

          <CodeBlock
            title="Python (redis-py)"
            language="python"
            code={`import redis

r = redis.Redis(host='127.0.0.1', port=6379)

r.set('user:1:name', 'Alice')
name = r.get('user:1:name')
print(name)  # b'Alice'

# Works the same as Redis — no code changes needed
r.hset('user:1', mapping={
    'name': 'Alice',
    'email': 'alice@example.com'
})
print(r.hgetall('user:1'))`}
          />

          <CodeBlock
            title="Node.js (ioredis)"
            language="javascript"
            code={`import Redis from 'ioredis';

const redis = new Redis({
  host: '127.0.0.1',
  port: 6379,
});

await redis.set('user:1:name', 'Alice');
const name = await redis.get('user:1:name');
console.log(name); // "Alice"

await redis.hset('user:1', { name: 'Alice', email: 'alice@example.com' });
const user = await redis.hgetall('user:1');
console.log(user); // { name: 'Alice', email: 'alice@example.com' }`}
          />
        </section>

        <section>
          <h2>5. Common Server Options</h2>
          <p>Some useful flags when starting the server:</p>
          <CodeBlock
            language="bash"
            code={`# Specify port and number of worker threads
justkv-server --port 6380 --threads 4

# Set a memory limit
justkv-server --maxmemory 4gb

# Bind to all interfaces (for remote access)
justkv-server --bind 0.0.0.0

# Use a configuration file
justkv-server --config /etc/justkv/justkv.conf`}
          />
          <p>
            See the{" "}
            <a
              href="/docs/configuration"
              className="text-primary hover:underline"
            >
              Configuration
            </a>{" "}
            guide for all available options.
          </p>
        </section>
      </div>

      <DocsPager currentHref="/docs/quickstart" />
    </div>
  );
}
