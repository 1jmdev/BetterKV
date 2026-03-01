import { Link } from "react-router-dom";
import {
  ArrowRight,
  Cpu,
  Database,
  Github,
  Lock,
  MemoryStick,
  Rocket,
  Zap,
} from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { CodeBlock } from "@/components/CodeBlock";

const features = [
  {
    icon: Database,
    title: "Redis Compatible",
    description:
      "Drop-in replacement using the same RESP protocol. Use your existing Redis clients, libraries, and tools without any code changes.",
  },
  {
    icon: Cpu,
    title: "Multi-Threaded",
    description:
      "Utilizes all available CPU cores for command processing. No more single-threaded bottlenecks limiting your throughput.",
  },
  {
    icon: Zap,
    title: "Built in Rust",
    description:
      "Memory-safe, zero-cost abstractions, and no garbage collector pauses. Predictable latency under load.",
  },
  {
    icon: MemoryStick,
    title: "Lower Memory Footprint",
    description:
      "Optimized data structures use 30-40% less memory than Redis for equivalent datasets, reducing infrastructure costs.",
  },
  {
    icon: Lock,
    title: "Open Source",
    description:
      "MIT licensed and community driven. Inspect the source, contribute features, or fork it for your own needs.",
  },
  {
    icon: Rocket,
    title: "Fast by Default",
    description:
      "Tuned out of the box for high throughput and low latency. No complex configuration needed to get great performance.",
  },
];

const stats = [
  { value: "312K", label: "ops/sec SET", change: "+27%" },
  { value: "398K", label: "ops/sec GET", change: "+28%" },
  { value: "0.28ms", label: "p99 latency", change: "-32%" },
  { value: "~35%", label: "less memory", change: "" },
];

export function LandingPage() {
  return (
    <div>
      {/* Hero */}
      <section className="relative overflow-hidden">
        {/* Background glow */}
        <div className="pointer-events-none absolute inset-0">
          <div className="absolute left-1/2 top-0 -translate-x-1/2 -translate-y-1/2">
            <div className="h-150 w-200 rounded-full bg-primary/[0.07] blur-[120px]" />
          </div>
        </div>

        <div className="relative mx-auto max-w-6xl px-4 pb-20 pt-24 sm:px-6 sm:pb-28 sm:pt-32">
          <div className="mx-auto max-w-3xl text-center">
            <Badge
              variant="outline"
              className="mb-6 border-primary/20 px-3 py-1 text-xs text-primary"
            >
              Now in public alpha
            </Badge>

            <h1 className="text-4xl font-bold tracking-tight sm:text-5xl lg:text-6xl">
              The Redis alternative,{" "}
              <span className="text-primary">built for speed</span>
            </h1>

            <p className="mt-6 text-lg leading-relaxed text-muted-foreground sm:text-xl">
              JustKV is a Redis-compatible key-value store written in Rust.
              Multi-threaded architecture, optimized memory usage, and the same
              protocol you already know.
            </p>

            <div className="mt-8 flex flex-col items-center justify-center gap-3 sm:flex-row">
              <Link
                to="/docs/quickstart"
                className="inline-flex h-9 items-center justify-center gap-2 rounded-lg bg-primary px-6 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
              >
                Get Started
                <ArrowRight className="size-4" />
              </Link>
              <a
                href="https://github.com/1jmdev/justkv"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex h-9 items-center justify-center gap-2 rounded-lg border border-border bg-background px-6 text-sm font-medium transition-colors hover:bg-muted"
              >
                <Github className="size-4" />
                View on GitHub
              </a>
            </div>

            <div className="mt-8">
              <CodeBlock
                code="curl -fsSL https://justkv.1jm.dev/install | sh"
                className="mx-auto max-w-md text-left"
              />
            </div>
          </div>
        </div>
      </section>

      {/* Stats bar */}
      <section className="border-y border-border/50 bg-card/30">
        <div className="mx-auto max-w-6xl px-4 py-8 sm:px-6">
          <div className="grid grid-cols-2 gap-6 md:grid-cols-4">
            {stats.map((stat) => (
              <div key={stat.label} className="text-center">
                <div className="text-2xl font-bold tracking-tight sm:text-3xl">
                  {stat.value}
                </div>
                <div className="mt-1 text-sm text-muted-foreground">
                  {stat.label}
                </div>
                {stat.change && (
                  <div className="mt-1 text-xs font-medium text-green-400">
                    {stat.change} vs Redis
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="mx-auto max-w-6xl px-4 py-20 sm:px-6 sm:py-28">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
            Why JustKV?
          </h2>
          <p className="mt-4 text-muted-foreground">
            Built from the ground up to solve the limitations of traditional
            key-value stores while staying compatible with the ecosystem you
            already use.
          </p>
        </div>

        <div className="mt-14 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {features.map((feature) => (
            <div
              key={feature.title}
              className="group rounded-xl border border-border/50 bg-card/40 p-6 transition-colors hover:border-border hover:bg-card/60"
            >
              <div className="flex size-10 items-center justify-center rounded-lg bg-primary/10">
                <feature.icon className="size-5 text-primary" />
              </div>
              <h3 className="mt-4 font-semibold">{feature.title}</h3>
              <p className="mt-2 text-sm leading-relaxed text-muted-foreground">
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </section>

      {/* Quick start */}
      <section className="border-t border-border/50 bg-card/20">
        <div className="mx-auto max-w-6xl px-4 py-20 sm:px-6 sm:py-28">
          <div className="mx-auto max-w-2xl text-center">
            <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
              Up and running in seconds
            </h2>
            <p className="mt-4 text-muted-foreground">
              Install JustKV, start the server, and connect with any Redis
              client. That's it.
            </p>
          </div>

          <div className="mx-auto mt-12 grid max-w-3xl gap-6">
            <div>
              <div className="mb-3 flex items-center gap-3">
                <span className="flex size-7 items-center justify-center rounded-full bg-primary/10 text-xs font-bold text-primary">
                  1
                </span>
                <span className="text-sm font-medium">Install JustKV</span>
              </div>
              <CodeBlock code="curl -fsSL https://justkv.1jm.dev/install | sh" />
            </div>

            <div>
              <div className="mb-3 flex items-center gap-3">
                <span className="flex size-7 items-center justify-center rounded-full bg-primary/10 text-xs font-bold text-primary">
                  2
                </span>
                <span className="text-sm font-medium">Start the server</span>
              </div>
              <CodeBlock code="justkv-server --port 6379 --threads 4" />
            </div>

            <div>
              <div className="mb-3 flex items-center gap-3">
                <span className="flex size-7 items-center justify-center rounded-full bg-primary/10 text-xs font-bold text-primary">
                  3
                </span>
                <span className="text-sm font-medium">
                  Connect with any Redis client
                </span>
              </div>
              <CodeBlock
                code={`redis-cli -p 6379
127.0.0.1:6379> SET hello "world"
OK
127.0.0.1:6379> GET hello
"world"`}
              />
            </div>
          </div>

          <div className="mt-12 text-center">
            <Link
              to="/docs/quickstart"
              className="inline-flex h-8 items-center justify-center gap-2 rounded-lg border border-border bg-background px-4 text-sm font-medium transition-colors hover:bg-muted"
            >
              Read the full quickstart guide
              <ArrowRight className="size-4" />
            </Link>
          </div>
        </div>
      </section>

      {/* Benchmark teaser */}
      <section className="mx-auto max-w-6xl px-4 py-20 sm:px-6 sm:py-28">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
            Measured, not claimed
          </h2>
          <p className="mt-4 text-muted-foreground">
            Transparent benchmarks on standardized hardware. No cherry-picked
            results.
          </p>
        </div>

        <div className="mx-auto mt-12 max-w-3xl">
          <div className="grid gap-4 sm:grid-cols-2">
            <BenchmarkBar
              label="SET throughput"
              justkvValue="312K ops/sec"
              redisValue="245K ops/sec"
              justkvPercent={100}
              redisPercent={78}
            />
            <BenchmarkBar
              label="Memory usage (1M keys)"
              justkvValue="1.2 GB"
              redisValue="1.8 GB"
              justkvPercent={67}
              redisPercent={100}
            />
            <BenchmarkBar
              label="p99 latency"
              justkvValue="0.28 ms"
              redisValue="0.41 ms"
              justkvPercent={68}
              redisPercent={100}
            />
            <BenchmarkBar
              label="GET throughput"
              justkvValue="398K ops/sec"
              redisValue="310K ops/sec"
              justkvPercent={100}
              redisPercent={78}
            />
          </div>

          <div className="mt-8 text-center">
            <Link
              to="/benchmarks"
              className="inline-flex h-8 items-center justify-center gap-2 rounded-lg border border-border bg-background px-4 text-sm font-medium transition-colors hover:bg-muted"
            >
              See detailed benchmarks
              <ArrowRight className="size-4" />
            </Link>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="border-t border-border/50">
        <div className="mx-auto max-w-6xl px-4 py-20 sm:px-6 sm:py-28">
          <div className="relative mx-auto max-w-2xl overflow-hidden rounded-2xl border border-border/50 bg-card/40 p-8 text-center sm:p-12">
            <div className="pointer-events-none absolute inset-0">
              <div className="absolute left-1/2 top-0 -translate-x-1/2 -translate-y-1/2">
                <div className="h-75 w-100 rounded-full bg-primary/6 blur-[80px]" />
              </div>
            </div>
            <div className="relative">
              <h2 className="text-2xl font-bold tracking-tight sm:text-3xl">
                Ready to switch?
              </h2>
              <p className="mt-3 text-muted-foreground">
                JustKV is open source and free to use. Get started in minutes
                with your existing Redis setup.
              </p>
              <div className="mt-6 flex flex-col items-center justify-center gap-3 sm:flex-row">
                <Link
                  to="/docs"
                  className="inline-flex h-9 items-center justify-center gap-2 rounded-lg bg-primary px-6 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90"
                >
                  Read the Docs
                  <ArrowRight className="size-4" />
                </Link>
                <a
                  href="https://github.com/1jmdev/justkv"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex h-9 items-center justify-center gap-2 rounded-lg border border-border bg-background px-6 text-sm font-medium transition-colors hover:bg-muted"
                >
                  <Github className="size-4" />
                  Star on GitHub
                </a>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}

function BenchmarkBar({
  label,
  justkvValue,
  redisValue,
  justkvPercent,
  redisPercent,
}: {
  label: string;
  justkvValue: string;
  redisValue: string;
  justkvPercent: number;
  redisPercent: number;
}) {
  return (
    <div className="rounded-xl border border-border/50 bg-card/40 p-6">
      <div className="text-sm font-medium text-muted-foreground">{label}</div>
      <div className="mt-3 space-y-3">
        <div>
          <div className="mb-1 flex justify-between text-sm">
            <span className="font-medium text-primary">JustKV</span>
            <span className="text-muted-foreground">{justkvValue}</span>
          </div>
          <div className="h-2 overflow-hidden rounded-full bg-muted">
            <div
              className="h-full rounded-full bg-primary transition-all duration-700"
              style={{ width: `${justkvPercent}%` }}
            />
          </div>
        </div>
        <div>
          <div className="mb-1 flex justify-between text-sm">
            <span className="font-medium text-muted-foreground">Redis</span>
            <span className="text-muted-foreground">{redisValue}</span>
          </div>
          <div className="h-2 overflow-hidden rounded-full bg-muted">
            <div
              className="h-full rounded-full bg-muted-foreground/40 transition-all duration-700"
              style={{ width: `${redisPercent}%` }}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
