import { motion } from "framer-motion"
import { PageHeader } from "@/components/layout/PageHeader"
import { Badge } from "@/components/ui/badge"
import { CheckIcon, LoaderIcon, CircleDotIcon } from "lucide-react"

const phases = [
  {
    status: "completed",
    title: "Core Engine",
    quarter: "Q4 2025",
    items: [
      "RESP3 protocol parser",
      "Multi-threaded command processing",
      "String, Hash, List, Set, Sorted Set data types",
      "Key expiration and eviction",
      "Basic AUTH support",
      "CLI and configuration system",
    ],
  },
  {
    status: "in-progress",
    title: "Production Readiness",
    quarter: "Q1 2026",
    items: [
      "TLS / mTLS support",
      "RDB persistence snapshots",
      "AOF append-only file logging",
      "Pub/Sub engine",
      "Transaction support (MULTI/EXEC)",
      "Cluster mode (multi-node)",
      "Comprehensive test suite",
    ],
  },
  {
    status: "planned",
    title: "Advanced Features",
    quarter: "Q2 2026",
    items: [
      "Lua scripting engine",
      "Stream data type",
      "ACL (Access Control Lists)",
      "Client-side caching support",
      "Replication",
      "Sentinel compatibility",
    ],
  },
  {
    status: "planned",
    title: "BetterKV Cloud",
    quarter: "Q3 2026",
    items: [
      "Managed hosting platform",
      "Auto-scaling",
      "Dashboard & monitoring",
      "Multi-region deployment",
      "Automated backups",
      "Usage-based billing",
    ],
  },
]

function StatusIcon({ status }: { status: string }) {
  switch (status) {
    case "completed":
      return <CheckIcon className="size-4 text-emerald-400" />
    case "in-progress":
      return <LoaderIcon className="size-4 text-primary animate-spin" />
    default:
      return <CircleDotIcon className="size-4 text-muted-foreground" />
  }
}

function StatusBadge({ status }: { status: string }) {
  switch (status) {
    case "completed":
      return <Badge variant="secondary" className="text-emerald-400">Completed</Badge>
    case "in-progress":
      return <Badge variant="secondary" className="text-primary">In Progress</Badge>
    default:
      return <Badge variant="secondary">Planned</Badge>
  }
}

const fadeUp = {
  initial: { opacity: 0, y: 20 },
  whileInView: { opacity: 1, y: 0 },
  viewport: { once: true, margin: "-100px" },
  transition: { duration: 0.5 },
}

export function RoadmapPage() {
  return (
    <div>
      <PageHeader
        badge="Roadmap"
        title="Where we're headed."
        description="Our development roadmap. Transparent, public, and constantly evolving."
      />

      <section className="py-24">
        <div className="mx-auto max-w-4xl px-6">
          <div className="relative space-y-8">
            <div className="absolute left-[19px] top-0 bottom-0 w-px bg-border/50" />

            {phases.map((phase, i) => (
              <motion.div
                key={phase.title}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true, margin: "-50px" }}
                transition={{ duration: 0.4, delay: i * 0.1 }}
                className="relative pl-12"
              >
                <div className="absolute left-0 top-6 flex size-10 items-center justify-center rounded-full border border-border/50 bg-card">
                  <StatusIcon status={phase.status} />
                </div>

                <div className="rounded-xl border border-border/50 bg-card p-6">
                  <div className="flex flex-wrap items-center gap-3">
                    <h3 className="text-lg font-semibold">{phase.title}</h3>
                    <StatusBadge status={phase.status} />
                    <span className="text-sm text-muted-foreground">{phase.quarter}</span>
                  </div>

                  <ul className="mt-4 grid gap-2 sm:grid-cols-2">
                    {phase.items.map((item) => (
                      <li key={item} className="flex items-center gap-2 text-sm text-muted-foreground">
                        <div className="size-1.5 rounded-full bg-primary/40" />
                        {item}
                      </li>
                    ))}
                  </ul>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </section>
    </div>
  )
}
