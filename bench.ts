// bench.ts
// Run JustKV vs Redis benchmarks (Bun-only), parse output, and generate bench.html with graphs.
// Usage (from repo root):
//   bun run bench.ts
//   bun run bench.ts --n 5000000 --c 16 --P 256 --host 127.0.0.1 --port 6379
//
// Requirements:
// - Start JustKV: target/release/justkv-server
// - Start Redis:  redis-server
// - Benchmark runner: target/release/justkv-benchmark
//
// Behavior:
// 1) start JustKV, wait 0.1s, run all tests, stop JustKV
// 2) start Redis,  wait 0.1s, run all tests, stop Redis
// 3) write bench.json + bench.html

type EngineName = "JustKV" | "Redis";

type BenchRunConfig = {
  host: string;
  port: number;
  n: number;
  c: number;
  P: number;
  warmup?: boolean;
  timeoutMsPerTest: number;
  waitAfterServerStartMs: number;
  outJson: string;
  outHtml: string;
  justkvServerCmd: string[];
  redisServerCmd: string[];
  benchCmd: string[]; // base command, without args
};

type ParsedBench = {
  test: string;
  header?: string;

  requests?: number;
  seconds?: number;

  clients?: number;
  payloadBytes?: number;
  pipeline?: number;

  latencyAvgMs?: number;
  latencyP50Ms?: number;
  latencyP95Ms?: number;
  latencyP99Ms?: number;

  rps?: number;

  raw: string;
  ok: boolean;
  error?: string;
  command: string[];
};

type BenchResult = {
  engine: EngineName;
  startedAt: string;
  finishedAt: string;
  config: Omit<BenchRunConfig, "justkvServerCmd" | "redisServerCmd" | "benchCmd">;
  runs: ParsedBench[];
};

const TESTS = [
  "PingInline",
  "PingMbulk",
  "Echo",
  "Set",
  "SetNx",
  "Get",
  "GetSet",
  "Mset",
  "Mget",
  "Del",
  "Exists",
  "Expire",
  "Ttl",
  "Incr",
  "IncrBy",
  "Decr",
  "DecrBy",
  "Strlen",
  "Eval",
  "EvalSha",
  "SetRange",
  "GetRange",
  "Lpush",
  "Rpush",
  "Lpop",
  "Rpop",
  "Llen",
  "Lrange",
  "Sadd",
  "Srem",
  "Scard",
  "Sismember",
  "Hset",
  "Hget",
  "Hgetall",
  "Hincrby",
  "Zadd",
  "Zrem",
  "Zcard",
  "Zscore",
  "Zrank",
  "Zrevrank",
] as const;

function parseArgs(argv: string[]): Partial<BenchRunConfig> {
  // Tiny arg parser: --key value (numbers auto-parse)
  const out: any = {};
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (!a.startsWith("--")) continue;
    const key = a.slice(2);
    const next = argv[i + 1];
    if (!next || next.startsWith("--")) {
      out[key] = true;
      continue;
    }
    i++;
    const num = Number(next);
    out[key] = Number.isFinite(num) && next.trim() !== "" ? num : next;
  }
  return out;
}

function sleep(ms: number) {
  return new Promise<void>((r) => setTimeout(r, ms));
}

function nowIso() {
  return new Date().toISOString();
}

function snakeCase(s: string): string {
  return s
    .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
    .replace(/[-\s]+/g, "_")
    .toLowerCase();
}

function kebabCase(s: string): string {
  return s
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .replace(/[_\s]+/g, "-")
    .toLowerCase();
}

function titleToCandidates(test: string): string[] {
  // We do not know which exact -t formatting the benchmark binary accepts.
  // So we try a small set of common enum/cli spellings until one works.
  const lower = test.toLowerCase();
  const snake = snakeCase(test);
  const kebab = kebabCase(test);

  // Some likely special-cases seen in redis-land (optional, low-risk):
  const compactLower = snake.replaceAll("_", ""); // e.g. get_set -> getset
  const compactKebab = kebab.replaceAll("-", ""); // e.g. get-set -> getset

  // Preserve order: most "human likely" first.
  const uniq: string[] = [];
  for (const v of [test, lower, snake, kebab, compactLower, compactKebab]) {
    if (!uniq.includes(v)) uniq.push(v);
  }
  return uniq;
}

function parseBenchmarkOutput(test: string, raw: string, ok: boolean, command: string[], error?: string): ParsedBench {
  // Example output:
  // ====== GET ======
  //   5000000 requests completed in 0.27 seconds
  //   16 parallel clients
  //   3 bytes payload
  //   256 pipeline depth
  //   latency avg/p50/p95/p99 = 0.0001/0.0008/0.0015/0.0022 ms
  //   18767889.98 requests per second

  const out: ParsedBench = { test, raw, ok, error, command };

  const header = raw.match(/=+\s*([A-Z0-9 _-]+)\s*=+/);
  if (header) out.header = header[1].trim();

  const reqLine = raw.match(/([\d_]+)\s+requests\s+completed\s+in\s+([\d.]+)\s+seconds/i);
  if (reqLine) {
    out.requests = Number(reqLine[1].replaceAll("_", ""));
    out.seconds = Number(reqLine[2]);
  }

  const clients = raw.match(/(\d+)\s+parallel\s+clients/i);
  if (clients) out.clients = Number(clients[1]);

  const payload = raw.match(/(\d+)\s+bytes\s+payload/i);
  if (payload) out.payloadBytes = Number(payload[1]);

  const pipe = raw.match(/(\d+)\s+pipeline\s+depth/i);
  if (pipe) out.pipeline = Number(pipe[1]);

  const lat = raw.match(/latency\s+avg\/p50\/p95\/p99\s*=\s*([\d.]+)\/([\d.]+)\/([\d.]+)\/([\d.]+)\s*ms/i);
  if (lat) {
    out.latencyAvgMs = Number(lat[1]);
    out.latencyP50Ms = Number(lat[2]);
    out.latencyP95Ms = Number(lat[3]);
    out.latencyP99Ms = Number(lat[4]);
  }

  const rps = raw.match(/([\d.]+)\s+requests\s+per\s+second/i);
  if (rps) out.rps = Number(rps[1]);

  // If it "ok" but missed the key line, mark not ok with a hint.
  if (ok && out.rps == null && out.requests == null) {
    out.ok = false;
    out.error = out.error ?? "Parse failed: expected benchmark lines not found in stdout/stderr.";
  }

  return out;
}

async function runCmdCapture(command: string[], opts: { cwd?: string; timeoutMs: number }): Promise<{ ok: boolean; stdout: string; stderr: string; code: number | null }> {
  const proc = Bun.spawn(command, {
    cwd: opts.cwd,
    stdout: "pipe",
    stderr: "pipe",
    stdin: "ignore",
  });

  const killTimer = setTimeout(() => {
    try {
      proc.kill("SIGKILL");
    } catch {}
  }, opts.timeoutMs);

  const [stdoutBuf, stderrBuf, exit] = await Promise.all([
    new Response(proc.stdout).arrayBuffer().catch(() => new ArrayBuffer(0)),
    new Response(proc.stderr).arrayBuffer().catch(() => new ArrayBuffer(0)),
    proc.exited,
  ]).finally(() => clearTimeout(killTimer));

  const stdout = new TextDecoder().decode(stdoutBuf);
  const stderr = new TextDecoder().decode(stderrBuf);
  const code = typeof exit === "number" ? exit : null;
  const ok = code === 0;

  return { ok, stdout, stderr, code };
}

async function startServer(command: string[], waitMs: number): Promise<any> {
  // Keep server output attached to terminal for debugging.
  // stdin:inherit lets sudo prompt for password if needed.
  const proc = Bun.spawn(command, {
    stdout: "ignore",
    stderr: "ignore",
    stdin: "ignore",
  });

  // Give it a moment to bind port, but fail fast if process dies immediately.
  const startup = await Promise.race([
    proc.exited.then((code) => ({ exited: true as const, code })),
    sleep(waitMs).then(() => ({ exited: false as const, code: null as number | null })),
  ]);

  if (startup.exited) {
    throw new Error(
      `Server exited before startup wait (${waitMs}ms), code=${startup.code}. Command: ${command.join(" ")}`
    );
  }

  return proc;
}

async function stopServer(proc: any, name: string) {
  // Gentle first, then firm.
  try {
    proc.kill("SIGTERM");
  } catch {}

  const deadline = Date.now() + 1200;
  while (Date.now() < deadline) {
    // exited is a Promise<number>
    const exited = await Promise.race([
      proc.exited.then(() => true).catch(() => true),
      sleep(100).then(() => false),
    ]);
    if (exited) return;
  }

  try {
    proc.kill("SIGKILL");
  } catch {}

  // Wait a beat so the OS can clean up the port.
  await sleep(150);

  // No throw: we prefer progress over drama.
  console.warn(`[warn] ${name} did not stop gracefully; SIGKILL sent.`);
}

async function runSingleTest(config: BenchRunConfig, test: string): Promise<ParsedBench> {
  const candidates = titleToCandidates(test);

  let lastStdout = "";
  let lastStderr = "";
  let lastCode: number | null = null;
  let lastCmd: string[] = [];
  let lastErr = "";

  for (const tValue of candidates) {
    const cmd = [
      ...config.benchCmd,
      "-t",
      tValue,
      "-n",
      String(config.n),
      "-c",
      String(config.c),
      "-P",
      String(config.P),
      "--host",
      config.host,
      "--port",
      String(config.port),
    ];
    lastCmd = cmd;

    const res = await runCmdCapture(cmd, { timeoutMs: config.timeoutMsPerTest });
    const combined = (res.stdout + "\n" + res.stderr).trim();

    lastStdout = res.stdout;
    lastStderr = res.stderr;
    lastCode = res.code;

    // Heuristic: some CLIs exit 0 but still print usage; some exit non-0.
    const looksLikeBench =
      /requests\s+completed\s+in\s+[\d.]+\s+seconds/i.test(combined) ||
      /requests\s+per\s+second/i.test(combined) ||
      /=+\s*[A-Z0-9 _-]+\s*=+/.test(combined);

    if (res.ok && looksLikeBench) {
      return parseBenchmarkOutput(test, combined, true, cmd);
    }

    // If this attempt clearly indicates unknown test type, keep trying.
    const unknownTest =
      /unknown|invalid|unrecognized|possible values|did you mean|error:\s*invalid value/i.test(combined);

    lastErr = combined || `exit code ${lastCode ?? "?"}`;

    if (!unknownTest) {
      // Not a "test name mismatch" problem; return the failure now.
      return parseBenchmarkOutput(test, combined, false, cmd, lastErr);
    }
  }

  const combined = (lastStdout + "\n" + lastStderr).trim();
  return parseBenchmarkOutput(
    test,
    combined || `Failed all -t candidates for ${test}. Last exit code: ${lastCode ?? "?"}`,
    false,
    lastCmd,
    lastErr || `Failed all -t candidates for ${test}.`
  );
}

async function runSuite(engine: EngineName, serverCmd: string[], config: BenchRunConfig): Promise<BenchResult> {
  const startedAt = nowIso();
  console.log(`\n🧪 Starting ${engine} server: ${serverCmd.join(" ")}`);
  const server = await startServer(serverCmd, config.waitAfterServerStartMs);

  const runs: ParsedBench[] = [];

  try {
    // Optional warmup: run GET once at small scale to stabilize JIT/caches.
    if (config.warmup) {
      console.log(`\n🔥 Warmup (${engine}): GET (quick)`);
      const warm = { ...config, n: Math.min(200_000, config.n), P: Math.min(64, config.P) };
      runs.push(await runSingleTest(warm, "Get"));
      await sleep(80);
    }

    for (const test of TESTS) {
      console.log(`\n▶ ${engine}: ${test}`);
      const parsed = await runSingleTest(config, test);
      runs.push(parsed);

      if (parsed.ok) {
        console.log(
          `   ✓ rps=${parsed.rps?.toFixed(2) ?? "?"}  p99=${parsed.latencyP99Ms ?? "?"}ms  seconds=${parsed.seconds ?? "?"}`
        );
      } else {
        console.log(`   ✗ failed: ${parsed.error ?? "unknown error"}`);
      }

      // tiny breath between tests
      await sleep(35);
    }
  } finally {
    console.log(`\n🧹 Stopping ${engine} server...`);
    await stopServer(server, engine);
  }

  const finishedAt = nowIso();
  return {
    engine,
    startedAt,
    finishedAt,
    config: {
      host: config.host,
      port: config.port,
      n: config.n,
      c: config.c,
      P: config.P,
      warmup: config.warmup,
      timeoutMsPerTest: config.timeoutMsPerTest,
      waitAfterServerStartMs: config.waitAfterServerStartMs,
      outJson: config.outJson,
      outHtml: config.outHtml,
    },
    runs,
  };
}

function htmlEscape(s: string) {
  return s
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

function buildHtml(results: BenchResult[], meta: { generatedAt: string }): string {
  // Prepare arrays for Chart.js
  const tests = [...TESTS];

  const byEngine = new Map<EngineName, Map<string, ParsedBench>>();
  for (const r of results) {
    const m = new Map<string, ParsedBench>();
    for (const run of r.runs) m.set(run.test, run);
    byEngine.set(r.engine, m);
  }

  function series(engine: EngineName, field: keyof ParsedBench): (number | null)[] {
    const m = byEngine.get(engine) ?? new Map();
    return tests.map((t) => {
      const v = m.get(t)?.[field] as any;
      return typeof v === "number" && Number.isFinite(v) ? v : null;
    });
  }

  const justkvRps = series("JustKV", "rps");
  const redisRps = series("Redis", "rps");
  const justkvP99 = series("JustKV", "latencyP99Ms");
  const redisP99 = series("Redis", "latencyP99Ms");

  const rows = tests
    .map((t) => {
      const j = byEngine.get("JustKV")?.get(t);
      const r = byEngine.get("Redis")?.get(t);

      const jRps = j?.rps ?? null;
      const rRps = r?.rps ?? null;

      const speedup =
        jRps != null && rRps != null && rRps !== 0 ? jRps / rRps : null;

      const jOk = j?.ok ? "ok" : "fail";
      const rOk = r?.ok ? "ok" : "fail";

      return `
        <tr>
          <td class="mono">${htmlEscape(t)}</td>
          <td class="${jOk} mono">${jRps == null ? "—" : jRps.toFixed(2)}</td>
          <td class="${rOk} mono">${rRps == null ? "—" : rRps.toFixed(2)}</td>
          <td class="mono">${speedup == null ? "—" : speedup.toFixed(3) + "×"}</td>
          <td class="${jOk} mono">${j?.latencyP99Ms == null ? "—" : j.latencyP99Ms.toFixed(4)}</td>
          <td class="${rOk} mono">${r?.latencyP99Ms == null ? "—" : r.latencyP99Ms.toFixed(4)}</td>
        </tr>
      `.trim();
    })
    .join("\n");

  // Embed JSON for drilldown
  const embeddedJson = htmlEscape(JSON.stringify({ meta, results }, null, 2));

  return `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>JustKV vs Redis Bench</title>
  <style>
    :root {
      color-scheme: dark;
      --bg: #0b0f14;
      --panel: #0f1722;
      --text: #e6edf3;
      --muted: #9aa7b2;
      --grid: rgba(255,255,255,0.08);
      --ok: #3ddc97;
      --fail: #ff6b6b;
      --border: rgba(255,255,255,0.10);
    }
    body {
      margin: 0;
      font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, "Apple Color Emoji","Segoe UI Emoji";
      background: radial-gradient(1200px 700px at 20% 0%, rgba(61,220,151,0.10), transparent 60%),
                  radial-gradient(900px 600px at 90% 20%, rgba(100,149,237,0.10), transparent 60%),
                  var(--bg);
      color: var(--text);
    }
    header {
      padding: 22px 18px 10px;
      border-bottom: 1px solid var(--border);
      background: rgba(0,0,0,0.20);
      backdrop-filter: blur(8px);
      position: sticky;
      top: 0;
      z-index: 10;
    }
    h1 {
      margin: 0 0 6px;
      font-size: 18px;
      font-weight: 700;
      letter-spacing: 0.2px;
    }
    .sub {
      margin: 0;
      color: var(--muted);
      font-size: 12px;
      line-height: 1.4;
    }
    main {
      padding: 16px 18px 40px;
      max-width: 1200px;
      margin: 0 auto;
    }
    .grid {
      display: grid;
      grid-template-columns: 1fr;
      gap: 14px;
    }
    @media (min-width: 960px) {
      .grid {
        grid-template-columns: 1fr 1fr;
      }
    }
    .card {
      background: rgba(15, 23, 34, 0.78);
      border: 1px solid var(--border);
      border-radius: 14px;
      padding: 14px;
      box-shadow: 0 10px 30px rgba(0,0,0,0.35);
    }
    .card h2 {
      margin: 0 0 10px;
      font-size: 13px;
      color: var(--muted);
      font-weight: 600;
      letter-spacing: 0.25px;
      text-transform: uppercase;
    }
    canvas { width: 100% !important; height: 360px !important; }
    table {
      width: 100%;
      border-collapse: collapse;
      font-size: 12px;
    }
    thead th {
      text-align: left;
      font-size: 11px;
      color: var(--muted);
      font-weight: 600;
      padding: 10px 8px;
      border-bottom: 1px solid var(--border);
    }
    tbody td {
      padding: 9px 8px;
      border-bottom: 1px solid rgba(255,255,255,0.06);
      vertical-align: top;
    }
    tbody tr:hover td {
      background: rgba(255,255,255,0.03);
    }
    .mono { font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; }
    .ok { color: var(--ok); }
    .fail { color: var(--fail); }
    details {
      margin-top: 14px;
    }
    summary {
      cursor: pointer;
      color: var(--muted);
      font-size: 12px;
    }
    pre {
      overflow: auto;
      padding: 12px;
      border-radius: 12px;
      border: 1px solid var(--border);
      background: rgba(0,0,0,0.25);
      max-height: 420px;
      font-size: 11px;
      line-height: 1.35;
    }
    .pill {
      display: inline-block;
      font-size: 11px;
      color: var(--muted);
      border: 1px solid var(--border);
      padding: 4px 8px;
      border-radius: 999px;
      margin-right: 6px;
      background: rgba(0,0,0,0.22);
    }
    .meta {
      margin-top: 8px;
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
    }
  </style>
</head>
<body>
  <header>
    <h1>JustKV vs Redis Bench</h1>
    <p class="sub">Generated ${htmlEscape(meta.generatedAt)}. RPS and p99 latency (ms) per command. 🧪</p>
  </header>
  <main>
    <div class="meta">
      <span class="pill">n=${htmlEscape(String(results[0]?.config.n ?? ""))}</span>
      <span class="pill">c=${htmlEscape(String(results[0]?.config.c ?? ""))}</span>
      <span class="pill">P=${htmlEscape(String(results[0]?.config.P ?? ""))}</span>
      <span class="pill">host=${htmlEscape(String(results[0]?.config.host ?? ""))}</span>
      <span class="pill">port=${htmlEscape(String(results[0]?.config.port ?? ""))}</span>
    </div>

    <div class="grid" style="margin-top:14px;">
      <div class="card">
        <h2>Requests per second (higher is better)</h2>
        <canvas id="rpsChart"></canvas>
      </div>

      <div class="card">
        <h2>p99 latency (ms, lower is better)</h2>
        <canvas id="p99Chart"></canvas>
      </div>
    </div>

    <div class="card" style="margin-top:14px;">
      <h2>Table</h2>
      <table>
        <thead>
          <tr>
            <th>Test</th>
            <th>JustKV RPS</th>
            <th>Redis RPS</th>
            <th>JustKV/Redis</th>
            <th>JustKV p99 (ms)</th>
            <th>Redis p99 (ms)</th>
          </tr>
        </thead>
        <tbody>
          ${rows}
        </tbody>
      </table>

      <details>
        <summary>Show embedded JSON (full raw outputs)</summary>
        <pre class="mono">${embeddedJson}</pre>
      </details>
    </div>
  </main>

  <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js"></script>
  <script>
    const tests = ${JSON.stringify(tests)};
    const justkvRps = ${JSON.stringify(justkvRps)};
    const redisRps  = ${JSON.stringify(redisRps)};
    const justkvP99 = ${JSON.stringify(justkvP99)};
    const redisP99  = ${JSON.stringify(redisP99)};

    function mkLine(ctx, title, aLabel, aData, bLabel, bData, yTitle) {
      return new Chart(ctx, {
        type: 'line',
        data: {
          labels: tests,
          datasets: [
            { label: aLabel, data: aData, tension: 0.25 },
            { label: bLabel, data: bData, tension: 0.25 },
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            title: { display: false, text: title },
            legend: { labels: { color: '#e6edf3' } },
            tooltip: { mode: 'index', intersect: false },
          },
          interaction: { mode: 'nearest', axis: 'x', intersect: false },
          scales: {
            x: {
              ticks: { color: '#9aa7b2', maxRotation: 70, minRotation: 40 },
              grid: { color: '${"#"}{getComputedStyle(document.documentElement).getPropertyValue("--grid")}' }
            },
            y: {
              title: { display: true, text: yTitle, color: '#9aa7b2' },
              ticks: { color: '#9aa7b2' },
              grid: { color: '${"#"}{getComputedStyle(document.documentElement).getPropertyValue("--grid")}' }
            }
          }
        }
      });
    }

    mkLine(document.getElementById('rpsChart'), 'RPS', 'JustKV', justkvRps, 'Redis', redisRps, 'requests/sec');
    mkLine(document.getElementById('p99Chart'), 'p99', 'JustKV', justkvP99, 'Redis', redisP99, 'ms');
  </script>
</body>
</html>`;
}

async function main() {
  const overrides = parseArgs(Bun.argv.slice(2));

  const config: BenchRunConfig = {
    host: String((overrides as any).host ?? "127.0.0.1"),
    port: Number((overrides as any).port ?? 6379),

    n: Number((overrides as any).n ?? 5_000_000),
    c: Number((overrides as any).c ?? 16),
    P: Number((overrides as any).P ?? 256),

    warmup: Boolean((overrides as any).warmup ?? false),

    // Realistic guardrails: a stuck test should not stall the universe.
    timeoutMsPerTest: Number((overrides as any).timeoutMsPerTest ?? 3 * 60_000),

    // User request: wait 0.1s then start benching.
    waitAfterServerStartMs: Number((overrides as any).waitAfterServerStartMs ?? 100),

    outJson: String((overrides as any).outJson ?? "bench.json"),
    outHtml: String((overrides as any).outHtml ?? "bench.html"),

    justkvServerCmd: ["target/release/justkv-server"],
    redisServerCmd: [
      "redis-server",
    ],

    // Benchmark runner (base). We add -t/-n/-c/-P/--host/--port per test.
    benchCmd: ["target/release/justkv-benchmark"],
  };

  console.log(`\n📌 Config: host=${config.host} port=${config.port} n=${config.n} c=${config.c} P=${config.P}`);
  console.log(`📄 Outputs: ${config.outJson}, ${config.outHtml}`);

  const justkv = await runSuite("JustKV", config.justkvServerCmd, config);
  // Give the OS a moment to release the port before Redis grabs it.
  await sleep(250);

  const redis = await runSuite("Redis", config.redisServerCmd, config);

  const meta = { generatedAt: nowIso() };
  const all = [justkv, redis];

  // Write JSON
  await Bun.write(config.outJson, JSON.stringify({ meta, results: all }, null, 2));

  // Write HTML
  const html = buildHtml(all, meta);
  await Bun.write(config.outHtml, html);

  // Print quick summary
  const okCount = (r: BenchResult) => r.runs.filter((x) => x.ok).length;
  console.log(`\n✅ Done.`);
  console.log(`   JustKV: ${okCount(justkv)}/${justkv.runs.length} ok`);
  console.log(`   Redis : ${okCount(redis)}/${redis.runs.length} ok`);
  console.log(`\n📄 Wrote ${config.outJson} and ${config.outHtml}`);
}

await main();
