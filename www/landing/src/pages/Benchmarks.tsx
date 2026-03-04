import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from "recharts";
import { PageHeader } from "@/components/PageHeader";
import {
  throughputData,
  memoryData,
  latencyData,
  testEnvironment,
} from "@/lib/benchmarks-data";

const JUSTKV_COLOR = "oklch(0.63 0.21 18)";
const REDIS_COLOR = "oklch(0.45 0.01 285)";

function formatNumber(n: number): string {
  if (n >= 1000000) return `${(n / 1000000).toFixed(1)}M`;
  if (n >= 1000) return `${(n / 1000).toFixed(0)}K`;
  return n.toString();
}

function CustomTooltip({
  active,
  payload,
  label,
}: {
  active?: boolean;
  payload?: Array<{ name: string; value: number; color: string }>;
  label?: string;
}) {
  if (!active || !payload?.length) return null;
  return (
    <div className="rounded-lg border border-border bg-card px-3 py-2 shadow-xl">
      <p className="mb-1 text-xs font-medium text-muted-foreground">{label}</p>
      {payload.map((entry) => (
        <p key={entry.name} className="text-sm" style={{ color: entry.color }}>
          {entry.name}: <span className="font-semibold">{typeof entry.value === 'number' && entry.value >= 1000 ? formatNumber(entry.value) : entry.value}</span>
        </p>
      ))}
    </div>
  );
}

export function BenchmarksPage() {
  return (
    <div className="mx-auto max-w-6xl px-4 py-12 sm:px-6 sm:py-16">
      <PageHeader
        title="Benchmarks"
        description="Performance comparison between JustKV and Redis on standardized hardware. All tests are reproducible."
      />

      {/* Key Metrics */}
      <div className="mt-12 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <MetricCard
          label="SET throughput"
          justkvValue="312K ops/sec"
          redisValue="245K ops/sec"
          improvement="+27%"
        />
        <MetricCard
          label="GET throughput"
          justkvValue="398K ops/sec"
          redisValue="310K ops/sec"
          improvement="+28%"
        />
        <MetricCard
          label="p99 latency"
          justkvValue="0.28 ms"
          redisValue="0.41 ms"
          improvement="-32%"
          lowerIsBetter
        />
        <MetricCard
          label="Memory (1M keys)"
          justkvValue="1.2 GB"
          redisValue="1.8 GB"
          improvement="-33%"
          lowerIsBetter
        />
      </div>

      {/* Throughput Chart */}
      <div className="mt-16">
        <h2 className="text-xl font-semibold">Throughput Comparison</h2>
        <p className="mt-1 text-sm text-muted-foreground">
          Operations per second across different command types. Higher is better.
        </p>
        <div className="mt-6 rounded-xl border border-border/50 bg-card/40 p-4 sm:p-6">
          <ResponsiveContainer width="100%" height={400}>
            <BarChart
              data={throughputData}
              margin={{ top: 5, right: 5, left: 5, bottom: 5 }}
            >
              <CartesianGrid
                strokeDasharray="3 3"
                stroke="oklch(1 0 0 / 6%)"
                vertical={false}
              />
              <XAxis
                dataKey="operation"
                tick={{ fill: "oklch(0.65 0.01 285)", fontSize: 12 }}
                axisLine={{ stroke: "oklch(1 0 0 / 8%)" }}
                tickLine={false}
              />
              <YAxis
                tickFormatter={formatNumber}
                tick={{ fill: "oklch(0.65 0.01 285)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
              />
              <Tooltip content={<CustomTooltip />} cursor={{ fill: "oklch(1 0 0 / 3%)" }} />
              <Legend
                wrapperStyle={{ fontSize: 12, paddingTop: 16 }}
              />
              <Bar
                dataKey="JustKV"
                fill={JUSTKV_COLOR}
                radius={[4, 4, 0, 0]}
                maxBarSize={40}
              />
              <Bar
                dataKey="Redis"
                fill={REDIS_COLOR}
                radius={[4, 4, 0, 0]}
                maxBarSize={40}
              />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Memory Chart */}
      <div className="mt-16">
        <h2 className="text-xl font-semibold">Memory Usage</h2>
        <p className="mt-1 text-sm text-muted-foreground">
          Memory consumed in MB for different dataset sizes (256-byte values).
          Lower is better.
        </p>
        <div className="mt-6 rounded-xl border border-border/50 bg-card/40 p-4 sm:p-6">
          <ResponsiveContainer width="100%" height={400}>
            <BarChart
              data={memoryData}
              margin={{ top: 5, right: 5, left: 5, bottom: 5 }}
            >
              <CartesianGrid
                strokeDasharray="3 3"
                stroke="oklch(1 0 0 / 6%)"
                vertical={false}
              />
              <XAxis
                dataKey="dataset"
                tick={{ fill: "oklch(0.65 0.01 285)", fontSize: 12 }}
                axisLine={{ stroke: "oklch(1 0 0 / 8%)" }}
                tickLine={false}
              />
              <YAxis
                tickFormatter={(v: number) => `${v} MB`}
                tick={{ fill: "oklch(0.65 0.01 285)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
              />
              <Tooltip content={<CustomTooltip />} cursor={{ fill: "oklch(1 0 0 / 3%)" }} />
              <Legend wrapperStyle={{ fontSize: 12, paddingTop: 16 }} />
              <Bar
                dataKey="JustKV"
                fill={JUSTKV_COLOR}
                radius={[4, 4, 0, 0]}
                maxBarSize={40}
              />
              <Bar
                dataKey="Redis"
                fill={REDIS_COLOR}
                radius={[4, 4, 0, 0]}
                maxBarSize={40}
              />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Latency Chart */}
      <div className="mt-16">
        <h2 className="text-xl font-semibold">Latency Distribution</h2>
        <p className="mt-1 text-sm text-muted-foreground">
          Latency in milliseconds at different percentiles under sustained load.
          Lower is better.
        </p>
        <div className="mt-6 rounded-xl border border-border/50 bg-card/40 p-4 sm:p-6">
          <ResponsiveContainer width="100%" height={350}>
            <BarChart
              data={latencyData}
              margin={{ top: 5, right: 5, left: 5, bottom: 5 }}
            >
              <CartesianGrid
                strokeDasharray="3 3"
                stroke="oklch(1 0 0 / 6%)"
                vertical={false}
              />
              <XAxis
                dataKey="percentile"
                tick={{ fill: "oklch(0.65 0.01 285)", fontSize: 12 }}
                axisLine={{ stroke: "oklch(1 0 0 / 8%)" }}
                tickLine={false}
              />
              <YAxis
                tickFormatter={(v: number) => `${v} ms`}
                tick={{ fill: "oklch(0.65 0.01 285)", fontSize: 12 }}
                axisLine={false}
                tickLine={false}
              />
              <Tooltip content={<CustomTooltip />} cursor={{ fill: "oklch(1 0 0 / 3%)" }} />
              <Legend wrapperStyle={{ fontSize: 12, paddingTop: 16 }} />
              <Bar
                dataKey="JustKV"
                fill={JUSTKV_COLOR}
                radius={[4, 4, 0, 0]}
                maxBarSize={40}
              />
              <Bar
                dataKey="Redis"
                fill={REDIS_COLOR}
                radius={[4, 4, 0, 0]}
                maxBarSize={40}
              />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* Test Environment */}
      <div className="mt-16">
        <h2 className="text-xl font-semibold">Test Environment</h2>
        <p className="mt-1 text-sm text-muted-foreground">
          All benchmarks were run on the same machine with the following
          configuration.
        </p>
        <div className="mt-6 grid gap-4 sm:grid-cols-2">
          <div className="rounded-xl border border-border/50 bg-card/40 p-6">
            <h3 className="text-sm font-semibold text-muted-foreground">
              Hardware
            </h3>
            <dl className="mt-3 space-y-2 text-sm">
              <div className="flex justify-between">
                <dt className="text-muted-foreground">CPU</dt>
                <dd>{testEnvironment.cpu}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Memory</dt>
                <dd>{testEnvironment.memory}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">OS</dt>
                <dd>{testEnvironment.os}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Kernel</dt>
                <dd>{testEnvironment.kernel}</dd>
              </div>
            </dl>
          </div>
          <div className="rounded-xl border border-border/50 bg-card/40 p-6">
            <h3 className="text-sm font-semibold text-muted-foreground">
              Test Parameters
            </h3>
            <dl className="mt-3 space-y-2 text-sm">
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Network</dt>
                <dd>{testEnvironment.network}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Clients</dt>
                <dd>{testEnvironment.clients}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Pipeline</dt>
                <dd>{testEnvironment.pipeline}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Data</dt>
                <dd>{testEnvironment.dataSize}</dd>
              </div>
            </dl>
          </div>
        </div>
      </div>

      {/* Methodology */}
      <div className="mt-16">
        <h2 className="text-xl font-semibold">Methodology</h2>
        <div className="mt-4 rounded-xl border border-border/50 bg-card/40 p-6 text-sm leading-relaxed text-muted-foreground space-y-3">
          <p>
            Benchmarks were conducted using <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">redis-benchmark</code> (included
            with Redis {testEnvironment.redisVersion}) for both Redis and JustKV to ensure
            identical test conditions.
          </p>
          <p>
            Each test was run 5 times with a 30-second warmup period. The
            reported numbers are the median of the 5 runs. Both servers were
            started with default configurations, except JustKV was configured
            with <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">--threads 8</code> to match the available CPU cores.
          </p>
          <p>
            Redis version: {testEnvironment.redisVersion}. JustKV version:{" "}
            {testEnvironment.justkvVersion}. Memory measurements taken via{" "}
            <code className="rounded bg-muted/60 px-1.5 py-0.5 text-xs text-primary">INFO memory</code> after populating the dataset and waiting 10
            seconds for stabilization.
          </p>
        </div>
      </div>

      {/* Versions */}
      <div className="mt-8 text-center text-xs text-muted-foreground/60">
        JustKV {testEnvironment.justkvVersion} vs Redis{" "}
        {testEnvironment.redisVersion} — Last updated February 2026
      </div>
    </div>
  );
}

function MetricCard({
  label,
  justkvValue,
  redisValue,
  improvement,
  lowerIsBetter = false,
}: {
  label: string;
  justkvValue: string;
  redisValue: string;
  improvement: string;
  lowerIsBetter?: boolean;
}) {
  return (
    <div className="rounded-xl border border-border/50 bg-card/40 p-5">
      <div className="text-xs font-medium uppercase tracking-wider text-muted-foreground">
        {label}
      </div>
      <div className="mt-2 text-2xl font-bold text-primary">{justkvValue}</div>
      <div className="mt-1 flex items-center gap-2 text-xs text-muted-foreground">
        <span>Redis: {redisValue}</span>
        <span
          className={
            lowerIsBetter ? "text-green-400" : "text-green-400"
          }
        >
          {improvement}
        </span>
      </div>
    </div>
  );
}
