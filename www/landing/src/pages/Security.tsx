import { motion } from "framer-motion";
import { PageHeader } from "@/components/layout/PageHeader";
import {
    ShieldCheckIcon,
    LockIcon,
    KeyIcon,
    EyeOffIcon,
    ServerIcon,
    BugIcon,
} from "lucide-react";

const securityFeatures = [
    {
        icon: LockIcon,
        title: "TLS / mTLS Encryption",
        description:
            "All client-server communication can be encrypted with TLS 1.3. Mutual TLS support ensures both sides verify each other's identity.",
    },
    {
        icon: KeyIcon,
        title: "Authentication",
        description:
            "Password-based AUTH and ACL support. Define per-user permissions to control who can read, write, or administer specific key patterns.",
    },
    {
        icon: EyeOffIcon,
        title: "No Undefined Behavior",
        description:
            "Built in Rust, which eliminates entire classes of vulnerabilities: buffer overflows, use-after-free, data races. Memory safety is guaranteed at compile time.",
    },
    {
        icon: ShieldCheckIcon,
        title: "Thread Safety",
        description:
            "Rust's ownership model ensures thread safety without runtime overhead. No race conditions, no data corruption under concurrent access.",
    },
    {
        icon: ServerIcon,
        title: "Network Isolation",
        description:
            "Bind to specific interfaces, configure allowed client ranges, and use protected mode to prevent accidental exposure to the public internet.",
    },
    {
        icon: BugIcon,
        title: "Security-First Development",
        description:
            "Regular dependency audits, fuzzing campaigns, and static analysis. Security issues are treated as P0 bugs with immediate response.",
    },
];

const fadeUp = {
    initial: { opacity: 0, y: 20 },
    whileInView: { opacity: 1, y: 0 },
    viewport: { once: true, margin: "-100px" },
    transition: { duration: 0.5 },
};

export function SecurityPage() {
    return (
        <div>
            <PageHeader
                badge="Security"
                title="Secure by design."
                description="Built in Rust for memory safety. Encrypted in transit. Authenticated at every layer."
            />

            <section className="border-b border-border/50 py-24">
                <div className="mx-auto max-w-6xl px-6">
                    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                        {securityFeatures.map((feature, i) => (
                            <motion.div
                                key={feature.title}
                                initial={{ opacity: 0, y: 20 }}
                                whileInView={{ opacity: 1, y: 0 }}
                                viewport={{ once: true, margin: "-50px" }}
                                transition={{ duration: 0.4, delay: i * 0.08 }}
                                className="rounded-xl border border-border/50 bg-card p-6"
                            >
                                <div className="mb-4 flex size-10 items-center justify-center rounded-lg bg-primary/10">
                                    <feature.icon className="size-5 text-primary" />
                                </div>
                                <h3 className="text-sm font-semibold">
                                    {feature.title}
                                </h3>
                                <p className="mt-2 text-sm text-muted-foreground leading-relaxed">
                                    {feature.description}
                                </p>
                            </motion.div>
                        ))}
                    </div>
                </div>
            </section>

            <section className="py-24">
                <div className="mx-auto max-w-4xl px-6">
                    <motion.div {...fadeUp}>
                        <h2 className="text-2xl font-bold tracking-tight">
                            Reporting Vulnerabilities
                        </h2>
                        <p className="mt-2 text-muted-foreground">
                            We take security seriously. If you discover a
                            vulnerability, please report it responsibly.
                        </p>
                    </motion.div>

                    <motion.div
                        {...fadeUp}
                        className="mt-8 rounded-xl border border-border/50 bg-card p-6"
                    >
                        <div className="space-y-4 text-sm text-muted-foreground leading-relaxed">
                            <p>
                                Email security reports to{" "}
                                <span className="font-mono text-primary">
                                    security@betterkv.com
                                </span>
                                . Include a detailed description of the
                                vulnerability, steps to reproduce, and any
                                relevant logs or screenshots.
                            </p>
                            <p>
                                We aim to acknowledge reports within 24 hours
                                and provide a resolution timeline within 72
                                hours. We will not take legal action against
                                researchers who follow responsible disclosure
                                practices.
                            </p>
                        </div>
                    </motion.div>
                </div>
            </section>
        </div>
    );
}
