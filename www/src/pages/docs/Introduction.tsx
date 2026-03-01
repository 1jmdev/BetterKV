import { Link } from "react-router-dom";
import { ArrowRight } from "lucide-react";
import { PageHeader } from "@/components/PageHeader";
import { DocsPager } from "@/components/DocsPager";

export function DocsIntroduction() {
  return (
    <div>
      <PageHeader
        title="Introduction"
        description="JustKV is a Redis-compatible key-value store built from the ground up in Rust for better performance and lower memory usage."
      />

      <div className="prose-docs mt-8 space-y-8">
        <section>
          <h2>What is JustKV?</h2>
          <p>
            JustKV is an open-source, in-memory key-value data store that speaks
            the Redis protocol (RESP). It's designed as a modern alternative to
            Redis, built in Rust with a multi-threaded architecture that takes
            advantage of all available CPU cores.
          </p>
          <p>
            Because JustKV implements the same wire protocol as Redis, you can
            use it with any existing Redis client library — no code changes
            required. Just point your application at a JustKV instance and it
            works.
          </p>
        </section>

        <section>
          <h2>Key Differentiators</h2>
          <div className="grid gap-4 not-prose sm:grid-cols-2">
            <div className="rounded-lg border border-border/50 bg-card/40 p-4">
              <h3 className="font-semibold text-sm">Multi-threaded by design</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                Redis processes commands on a single thread. JustKV distributes
                work across all available cores, eliminating the single-threaded
                bottleneck for CPU-bound workloads.
              </p>
            </div>
            <div className="rounded-lg border border-border/50 bg-card/40 p-4">
              <h3 className="font-semibold text-sm">Lower memory overhead</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                Rust's ownership model and optimized internal data structures
                reduce per-key memory overhead by 30-40% compared to Redis for
                typical workloads.
              </p>
            </div>
            <div className="rounded-lg border border-border/50 bg-card/40 p-4">
              <h3 className="font-semibold text-sm">No garbage collector</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                Built in Rust with no runtime GC. This means no GC pauses and
                more predictable tail latency, especially under high throughput.
              </p>
            </div>
            <div className="rounded-lg border border-border/50 bg-card/40 p-4">
              <h3 className="font-semibold text-sm">Wire-compatible</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                Implements the RESP protocol, so every Redis client library
                works out of the box. Switch your connection string, not your
                code.
              </p>
            </div>
          </div>
        </section>

        <section>
          <h2>When to use JustKV</h2>
          <p>JustKV is a good fit when you need:</p>
          <ul>
            <li>
              An in-memory cache or session store with Redis compatibility
            </li>
            <li>
              Higher throughput than a single Redis instance without clustering
            </li>
            <li>
              Lower memory usage for large datasets to reduce infrastructure
              costs
            </li>
            <li>
              More predictable latency without GC-related tail latency spikes
            </li>
          </ul>
        </section>

        <section>
          <h2>Current Status</h2>
          <p>
            JustKV is currently in <strong>public alpha</strong>. The core
            data types and most common Redis commands are implemented. See the{" "}
            <Link
              to="/docs/compatibility"
              className="text-primary hover:underline"
            >
              Redis Compatibility
            </Link>{" "}
            page for details on which commands are currently supported.
          </p>
          <p>
            We recommend evaluating JustKV for caching and session storage
            workloads. Persistence features (RDB/AOF equivalent) are on the
            roadmap.
          </p>
        </section>

        <section>
          <h2>Next Steps</h2>
          <div className="not-prose flex flex-col gap-2 sm:flex-row">
            <Link
              to="/docs/installation"
              className="inline-flex h-8 items-center gap-2 rounded-lg border border-border bg-background px-4 text-sm font-medium transition-colors hover:bg-muted"
            >
              Install JustKV
              <ArrowRight className="size-3.5" />
            </Link>
            <Link
              to="/docs/quickstart"
              className="inline-flex h-8 items-center gap-2 rounded-lg bg-primary px-4 text-sm font-medium text-primary-foreground! transition-colors hover:bg-primary/90"
            >
              Quickstart Guide
              <ArrowRight className="size-3.5" />
            </Link>
          </div>
        </section>
      </div>

      <DocsPager currentHref="/docs" />
    </div>
  );
}
