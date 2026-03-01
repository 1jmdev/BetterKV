import { PageHeader } from "@/components/PageHeader";
import { CodeBlock } from "@/components/CodeBlock";
import { DocsPager } from "@/components/DocsPager";

export function DocsInstallation() {
  return (
    <div>
      <PageHeader
        title="Installation"
        description="Install JustKV on your system using one of the methods below."
      />

      <div className="prose-docs mt-8 space-y-8">
        <section>
          <h2>Quick Install (Linux & macOS)</h2>
          <p>
            The fastest way to install JustKV is using the install script. This
            detects your platform and downloads the correct prebuilt binary.
          </p>
          <CodeBlock language="bash" code="curl -fsSL https://justkv.1jm.dev/install | sh" />
          <p>
            This installs <code>justkv-server</code> and{" "}
            <code>justkv-cli</code> to <code>/usr/local/bin</code>. You may
            need <code>sudo</code> depending on your system configuration.
          </p>
        </section>

        <section>
          <h2>Docker</h2>
          <p>
            JustKV is available as a Docker image. This is the easiest way to
            try it without installing anything on your system.
          </p>
          <CodeBlock
            language="bash"
            code={`# Pull and run with default settings
docker run -d --name justkv -p 6379:6379 ghcr.io/1jmdev/justkv:latest

# Run with custom configuration
docker run -d --name justkv \\
  -p 6379:6379 \\
  -v ./data:/data \\
  ghcr.io/1jmdev/justkv:latest \\
  --threads 4 --maxmemory 2gb`}
          />
        </section>

        <section>
          <h2>Build from Source</h2>
          <p>
            JustKV requires Rust 1.75 or later. Clone the repository and build
            with Cargo:
          </p>
          <CodeBlock
            language="bash"
            code={`git clone https://github.com/1jmdev/justkv.git
cd justkv
cargo build --release`}
            showLineNumbers
          />
          <p>
            The compiled binaries will be in{" "}
            <code>target/release/justkv-server</code> and{" "}
            <code>target/release/justkv-cli</code>.
          </p>
        </section>

        <section>
          <h2>Package Managers</h2>

          <h3>Homebrew (macOS)</h3>
          <CodeBlock
            language="bash"
            code={`brew tap 1jmdev/justkv
brew install justkv`}
          />

          <h3>Cargo</h3>
          <p>
            If you have a Rust toolchain installed, you can install directly
            from crates.io:
          </p>
          <CodeBlock language="bash" code="cargo install justkv" />
        </section>

        <section>
          <h2>Verify Installation</h2>
          <p>After installing, verify that JustKV is available:</p>
          <CodeBlock
            language="bash"
            code={`$ justkv-server --version
justkv-server 0.1.0

$ justkv-cli --version
justkv-cli 0.1.0`}
          />
        </section>

        <section>
          <h2>System Requirements</h2>
          <div className="not-prose overflow-hidden rounded-lg border border-border/50">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-border/50 bg-card/40">
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Requirement
                  </th>
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Minimum
                  </th>
                  <th className="px-4 py-2 text-left font-medium text-muted-foreground">
                    Recommended
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2 font-medium">OS</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Linux, macOS
                  </td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Linux (kernel 5.10+)
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2 font-medium">CPU</td>
                  <td className="px-4 py-2 text-muted-foreground">1 core</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    4+ cores
                  </td>
                </tr>
                <tr className="border-b border-border/50">
                  <td className="px-4 py-2 font-medium">RAM</td>
                  <td className="px-4 py-2 text-muted-foreground">256 MB</td>
                  <td className="px-4 py-2 text-muted-foreground">
                    Depends on dataset
                  </td>
                </tr>
                <tr>
                  <td className="px-4 py-2 font-medium">Rust (build only)</td>
                  <td className="px-4 py-2 text-muted-foreground">1.75+</td>
                  <td className="px-4 py-2 text-muted-foreground">Latest stable</td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>

      <DocsPager currentHref="/docs/installation" />
    </div>
  );
}
