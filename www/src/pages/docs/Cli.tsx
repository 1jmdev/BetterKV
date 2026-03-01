import { PageHeader } from "@/components/PageHeader";
import { CodeBlock } from "@/components/CodeBlock";
import { DocsPager } from "@/components/DocsPager";

export function DocsCli() {
  return (
    <div>
      <PageHeader
        title="CLI Reference"
        description="Command-line options for the JustKV server and client binaries."
      />

      <div className="prose-docs mt-8 space-y-8">
        <section>
          <h2>justkv-server</h2>
          <p>The main server binary. Starts the JustKV key-value store.</p>
          <CodeBlock
            language="plain"
            code={`justkv-server [OPTIONS]

OPTIONS:
  --config <FILE>       Path to configuration file
  --port <PORT>         TCP port to listen on [default: 6379]
  --bind <ADDR>         Address to bind to [default: 127.0.0.1]
  --threads <N>         Worker threads [default: auto]
  --io-threads <N>      I/O threads [default: auto]
  --maxmemory <SIZE>    Memory limit (e.g. 4gb, 512mb) [default: unlimited]
  --maxmemory-policy <POLICY>
                        Eviction policy [default: noeviction]
  --loglevel <LEVEL>    Log level: debug, info, warn, error [default: info]
  --logfile <FILE>      Log file path [default: stderr]
  --requirepass <PASS>  Require password for connections
  --daemonize           Run as background daemon
  --pidfile <FILE>      PID file path (used with --daemonize)
  --version             Print version and exit
  --help                Print help and exit`}
          />

          <h3>Examples</h3>
          <CodeBlock
            language="bash"
            code={`# Start with defaults (port 6379, all cores)
justkv-server

# Custom port with 4 threads and 2GB memory limit
justkv-server --port 6380 --threads 4 --maxmemory 2gb

# Use a config file
justkv-server --config /etc/justkv/justkv.conf

# Run as a daemon
justkv-server --daemonize --pidfile /var/run/justkv.pid --logfile /var/log/justkv.log

# Bind to all interfaces with password
justkv-server --bind 0.0.0.0 --requirepass mysecretpassword`}
          />
        </section>

        <section>
          <h2>justkv-cli</h2>
          <p>
            Interactive command-line client for JustKV. Compatible with{" "}
            <code>redis-cli</code> usage patterns.
          </p>
          <CodeBlock
            language="plain"
            code={`justkv-cli [OPTIONS] [COMMAND [ARGS...]]

OPTIONS:
  -h, --host <HOST>     Server hostname [default: 127.0.0.1]
  -p, --port <PORT>     Server port [default: 6379]
  -a, --auth <PASS>     Password for authentication
  -n, --db <NUM>        Database number [default: 0]
  --raw                 Use raw output format (no quoting)
  --csv                 Output in CSV format
  --json                Output in JSON format
  --latency             Enter latency monitoring mode
  --stat                Print rolling server stats
  --scan                Iterate over keys using SCAN
  --pattern <PATTERN>   Pattern for --scan [default: *]
  --pipe                Transfer data from stdin (mass insert)
  --version             Print version and exit
  --help                Print help and exit`}
          />

          <h3>Interactive Mode</h3>
          <p>Run without arguments to enter interactive mode:</p>
          <CodeBlock
            language="bash"
            code={`$ justkv-cli
127.0.0.1:6379> PING
PONG
127.0.0.1:6379> SET mykey "Hello"
OK
127.0.0.1:6379> GET mykey
"Hello"`}
          />

          <h3>Single Command Mode</h3>
          <p>
            Pass a command directly to execute it and exit:
          </p>
          <CodeBlock
            language="bash"
            code={`# Execute a single command
$ justkv-cli SET mykey "Hello"
OK

$ justkv-cli GET mykey
"Hello"

# Connect to a specific host and port
$ justkv-cli -h 10.0.0.5 -p 6380 INFO server

# Authenticate and run a command
$ justkv-cli -a mysecretpassword DBSIZE
(integer) 42`}
          />

          <h3>Monitoring & Diagnostics</h3>
          <CodeBlock
            language="bash"
            code={`# Monitor latency
$ justkv-cli --latency
min: 0, max: 1, avg: 0.12 (1523 samples)

# Rolling server statistics
$ justkv-cli --stat
------- data ------ --------------------- load ----------------------
keys       mem      clients blocked  requests     connections
42         1.20M    3       0        1205 (+0)    12

# Scan all keys matching a pattern
$ justkv-cli --scan --pattern "user:*"
user:1
user:2
user:3`}
          />
        </section>

        <section>
          <h2>Server Management Commands</h2>
          <p>
            These commands can be run through the CLI to manage a running
            server:
          </p>
          <div className="not-prose overflow-hidden rounded-lg border border-border/50">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-border/50 bg-card/40">
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Command
                  </th>
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Description
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      INFO [section]
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Server information. Sections: server, memory, clients, stats
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      DBSIZE
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Number of keys in the current database
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      FLUSHDB
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Remove all keys from the current database
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      FLUSHALL
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Remove all keys from all databases
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      CONFIG GET &lt;param&gt;
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Get runtime configuration parameter
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      CONFIG SET &lt;param&gt; &lt;value&gt;
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Set runtime configuration parameter
                  </td>
                </tr>
                <tr>
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      SHUTDOWN
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Gracefully shut down the server
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>

      <DocsPager currentHref="/docs/cli" />
    </div>
  );
}
