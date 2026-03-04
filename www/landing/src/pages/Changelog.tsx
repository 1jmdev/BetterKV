import { motion } from "framer-motion"
import { PageHeader } from "@/components/layout/PageHeader"
import { Badge } from "@/components/ui/badge"

const releases = [
  {
    version: "0.3.0",
    date: "February 28, 2026",
    tag: "Latest",
    changes: [
      { type: "feature", text: "Pub/Sub support with SUBSCRIBE, PUBLISH, PSUBSCRIBE" },
      { type: "feature", text: "Transaction support with MULTI, EXEC, DISCARD" },
      { type: "improvement", text: "30% improvement in pipeline throughput" },
      { type: "improvement", text: "Reduced memory overhead per key by 15%" },
      { type: "fix", text: "Fixed edge case in EXPIRE with negative TTL values" },
      { type: "fix", text: "Fixed RESP3 parsing for bulk strings with CRLF content" },
    ],
  },
  {
    version: "0.2.0",
    date: "January 15, 2026",
    changes: [
      { type: "feature", text: "Sorted Set data type with full command support" },
      { type: "feature", text: "RDB persistence snapshots" },
      { type: "feature", text: "AOF append-only file logging" },
      { type: "feature", text: "TLS support for client connections" },
      { type: "improvement", text: "Optimized hash table resizing under load" },
      { type: "improvement", text: "Better error messages for invalid commands" },
      { type: "fix", text: "Fixed memory leak in long-running SCAN iterations" },
    ],
  },
  {
    version: "0.1.0",
    date: "December 1, 2025",
    changes: [
      { type: "feature", text: "Initial release" },
      { type: "feature", text: "String, Hash, List, Set data types" },
      { type: "feature", text: "RESP3 protocol support" },
      { type: "feature", text: "Multi-threaded command processing" },
      { type: "feature", text: "Key expiration and TTL" },
      { type: "feature", text: "Basic AUTH authentication" },
      { type: "feature", text: "Configuration file support" },
    ],
  },
]

function ChangeTypeBadge({ type }: { type: string }) {
  switch (type) {
    case "feature":
      return <Badge variant="secondary" className="text-primary">New</Badge>
    case "improvement":
      return <Badge variant="secondary" className="text-amber-400">Improved</Badge>
    case "fix":
      return <Badge variant="secondary" className="text-emerald-400">Fixed</Badge>
    default:
      return <Badge variant="secondary">{type}</Badge>
  }
}

const fadeUp = {
  initial: { opacity: 0, y: 20 },
  whileInView: { opacity: 1, y: 0 },
  viewport: { once: true, margin: "-100px" },
  transition: { duration: 0.5 },
}

export function ChangelogPage() {
  return (
    <div>
      <PageHeader
        badge="Changelog"
        title="Release history."
        description="Everything that's shipped. Every version, every change."
      />

      <section className="py-24">
        <div className="mx-auto max-w-4xl px-6 space-y-12">
          {releases.map((release, i) => (
            <motion.div
              key={release.version}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true, margin: "-50px" }}
              transition={{ duration: 0.4, delay: i * 0.1 }}
              className="rounded-xl border border-border/50 bg-card p-6"
            >
              <div className="flex flex-wrap items-center gap-3">
                <h2 className="text-xl font-bold font-mono">v{release.version}</h2>
                {release.tag && <Badge variant="default">{release.tag}</Badge>}
                <span className="text-sm text-muted-foreground">{release.date}</span>
              </div>

              <ul className="mt-5 space-y-3">
                {release.changes.map((change, j) => (
                  <li key={j} className="flex items-start gap-3 text-sm">
                    <ChangeTypeBadge type={change.type} />
                    <span className="text-muted-foreground">{change.text}</span>
                  </li>
                ))}
              </ul>
            </motion.div>
          ))}
        </div>
      </section>
    </div>
  )
}
