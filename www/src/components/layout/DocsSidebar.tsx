import { useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { ChevronDown, Menu, X } from "lucide-react";
import { cn } from "@/lib/utils";
import { docsNav } from "@/lib/docs-nav";
import { Button } from "@/components/ui/button";

export function DocsSidebar() {
  const location = useLocation();
  const [mobileOpen, setMobileOpen] = useState(false);

  const sidebar = (
    <nav className="space-y-6">
      {docsNav.map((section) => (
        <div key={section.title}>
          <div className="flex items-center gap-1 text-xs font-semibold uppercase tracking-wider text-muted-foreground/70">
            <ChevronDown className="size-3" />
            {section.title}
          </div>
          <ul className="mt-2 space-y-0.5">
            {section.items.map((item) => (
              <li key={item.href}>
                <Link
                  to={item.href}
                  onClick={() => setMobileOpen(false)}
                  className={cn(
                    "block rounded-md px-3 py-1.5 text-sm transition-colors",
                    location.pathname === item.href
                      ? "bg-primary/10 font-medium text-primary"
                      : "text-muted-foreground hover:bg-accent/50 hover:text-foreground"
                  )}
                >
                  {item.title}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      ))}
    </nav>
  );

  return (
    <>
      {/* Mobile toggle */}
      <div className="fixed bottom-4 right-4 z-40 lg:hidden">
        <Button
          size="icon"
          className="size-10 rounded-full shadow-lg"
          onClick={() => setMobileOpen(!mobileOpen)}
          aria-label="Toggle docs navigation"
        >
          {mobileOpen ? <X className="size-4" /> : <Menu className="size-4" />}
        </Button>
      </div>

      {/* Mobile sidebar */}
      {mobileOpen && (
        <div className="fixed inset-0 z-30 lg:hidden">
          <div
            className="absolute inset-0 bg-background/80 backdrop-blur-sm"
            onClick={() => setMobileOpen(false)}
          />
          <div className="absolute inset-y-0 left-0 w-72 overflow-y-auto border-r border-border bg-background p-6 pt-20">
            {sidebar}
          </div>
        </div>
      )}

      {/* Desktop sidebar */}
      <aside className="hidden w-56 shrink-0 lg:block">
        <div className="sticky top-20">{sidebar}</div>
      </aside>
    </>
  );
}
