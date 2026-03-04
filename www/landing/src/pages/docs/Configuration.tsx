import { PageHeader } from "@/components/PageHeader";
import { CodeBlock } from "@/components/CodeBlock";
import { DocsPager } from "@/components/DocsPager";

export function DocsConfiguration() {
  return (
    <div>
      <PageHeader
        title="Configuration"
        description="Configure JustKV using a config file, command-line flags, or environment variables."
      />

      <div className="prose-docs mt-8 space-y-8">
        <section>
          <h2>Configuration File</h2>
          <p>
            JustKV uses a simple key-value configuration file. By default, it
            looks for <code>justkv.conf</code> in the current directory. You can
            specify a different path with the <code>--config</code> flag.
          </p>
          <CodeBlock
            title="justkv.conf"
            language="ini"
            showLineNumbers
            code={`# Network
bind 127.0.0.1
port 6379

# Performance
threads auto
io-threads 4

# Memory
maxmemory 4gb
maxmemory-policy allkeys-lru

# Logging
loglevel info
logfile /var/log/justkv/justkv.log

# Security
requirepass ""

# Snapshotting (experimental)
# save 900 1
# save 300 10
# dir /var/lib/justkv`}
          />
        </section>

        <section>
          <h2>Configuration Reference</h2>

          <h3>Network Settings</h3>
          <div className="not-prose overflow-hidden rounded-lg border border-border/50">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-border/50 bg-card/40">
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Option
                  </th>
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Default
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
                      bind
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    127.0.0.1
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Address to bind the server to. Use <code>0.0.0.0</code> for
                    all interfaces.
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      port
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">6379</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    TCP port to listen on.
                  </td>
                </tr>
                <tr>
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      tcp-backlog
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">511</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    TCP listen backlog size.
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <h3>Performance Settings</h3>
          <div className="not-prose overflow-hidden rounded-lg border border-border/50">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-border/50 bg-card/40">
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Option
                  </th>
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Default
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
                      threads
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">auto</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Number of worker threads. <code>auto</code> uses all
                    available cores.
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      io-threads
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">auto</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Number of I/O threads for network operations.
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <h3>Memory Settings</h3>
          <div className="not-prose overflow-hidden rounded-lg border border-border/50">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-border/50 bg-card/40">
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Option
                  </th>
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Default
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
                      maxmemory
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">0 (unlimited)</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Maximum memory limit. Supports <code>mb</code>,{" "}
                    <code>gb</code> suffixes.
                  </td>
                </tr>
                <tr>
                  <td className="px-4 py-2">
                    <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">
                      maxmemory-policy
                    </code>
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">noeviction</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Eviction policy when maxmemory is reached. Options:{" "}
                    <code>noeviction</code>, <code>allkeys-lru</code>,{" "}
                    <code>volatile-lru</code>, <code>allkeys-random</code>,{" "}
                    <code>volatile-random</code>, <code>volatile-ttl</code>.
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

        <section>
          <h2>Environment Variables</h2>
          <p>
            All configuration options can be set via environment variables using
            the <code>JUSTKV_</code> prefix with uppercase names:
          </p>
          <CodeBlock
            language="bash"
            code={`export JUSTKV_PORT=6380
export JUSTKV_THREADS=4
export JUSTKV_MAXMEMORY=4gb
export JUSTKV_BIND=0.0.0.0
export JUSTKV_LOGLEVEL=debug

justkv-server`}
          />
          <p>
            Environment variables take precedence over config file values,
            and command-line flags take precedence over both.
          </p>
        </section>

        <section>
          <h2>Precedence Order</h2>
          <p>Configuration values are resolved in this order (highest first):</p>
          <ol>
            <li>Command-line flags (<code>--port 6380</code>)</li>
            <li>
              Environment variables (<code>JUSTKV_PORT=6380</code>)
            </li>
            <li>
              Configuration file (<code>port 6380</code>)
            </li>
            <li>Built-in defaults</li>
          </ol>
        </section>
      </div>

      <DocsPager currentHref="/docs/configuration" />
    </div>
  );
}
