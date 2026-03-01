import { Link } from "react-router-dom";
import { Database } from "lucide-react";

export function Footer() {
  return (
    <footer className="border-t border-border/50 bg-background">
      <div className="mx-auto max-w-6xl px-4 py-12 sm:px-6">
        <div className="grid grid-cols-2 gap-8 md:grid-cols-4">
          {/* Brand */}
          <div className="col-span-2 md:col-span-1">
            <Link
              to="/"
              className="flex items-center gap-2 font-semibold tracking-tight"
            >
              <Database className="size-5 text-primary" />
              <span>JustKV</span>
            </Link>
            <p className="mt-3 text-sm text-muted-foreground">
              A Redis-compatible key-value store built in Rust. Open source,
              multi-threaded, and optimized for performance.
            </p>
          </div>

          {/* Docs */}
          <div>
            <h3 className="text-sm font-semibold">Documentation</h3>
            <ul className="mt-3 space-y-2">
              <li>
                <Link
                  to="/docs"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Introduction
                </Link>
              </li>
              <li>
                <Link
                  to="/docs/installation"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Installation
                </Link>
              </li>
              <li>
                <Link
                  to="/docs/quickstart"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Quickstart
                </Link>
              </li>
              <li>
                <Link
                  to="/docs/compatibility"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Redis Compatibility
                </Link>
              </li>
            </ul>
          </div>

          {/* Project */}
          <div>
            <h3 className="text-sm font-semibold">Project</h3>
            <ul className="mt-3 space-y-2">
              <li>
                <Link
                  to="/benchmarks"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Benchmarks
                </Link>
              </li>
              <li>
                <a
                  href="https://github.com/1jmdev/justkv"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  GitHub
                </a>
              </li>
              <li>
                <a
                  href="https://github.com/1jmdev/justkv/releases"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Releases
                </a>
              </li>
              <li>
                <a
                  href="https://github.com/1jmdev/justkv/blob/main/LICENSE"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  MIT License
                </a>
              </li>
            </ul>
          </div>

          {/* Community */}
          <div>
            <h3 className="text-sm font-semibold">Community</h3>
            <ul className="mt-3 space-y-2">
              <li>
                <a
                  href="https://github.com/1jmdev/justkv/issues"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Issues
                </a>
              </li>
              <li>
                <a
                  href="https://github.com/1jmdev/justkv/discussions"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Discussions
                </a>
              </li>
              <li>
                <a
                  href="https://github.com/1jmdev/justkv/blob/main/CONTRIBUTING.md"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-muted-foreground transition-colors hover:text-foreground"
                >
                  Contributing
                </a>
              </li>
            </ul>
          </div>
        </div>

        <div className="mt-10 border-t border-border/50 pt-6">
          <p className="text-center text-xs text-muted-foreground">
            &copy; {new Date().getFullYear()} JustKV Contributors. Released
            under the MIT License.
          </p>
        </div>
      </div>
    </footer>
  );
}
