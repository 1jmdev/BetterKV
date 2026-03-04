import { Link } from "react-router-dom"
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu"
import { Button } from "@/components/ui/button"
import { WaitlistModal } from "@/components/layout/WaitlistModal"
import {
  GaugeIcon,
  ShieldCheckIcon,
  ArrowLeftRightIcon,
  MapIcon,
  TagIcon,
  BookOpenIcon,
  DownloadIcon,
  GitBranchIcon,
  FileTextIcon,
  UsersIcon,
  ScaleIcon,
  LayersIcon,
  DatabaseIcon,
  UserIcon,
  BarChart3Icon,
  MailIcon,
  FlagIcon,
  GamepadIcon,
  ZapIcon,
} from "lucide-react"

const productLinks = [
  { to: "/performance", label: "Performance", description: "Benchmarks & architecture", icon: GaugeIcon },
  { to: "/compatibility", label: "Compatibility", description: "Redis protocol support", icon: ArrowLeftRightIcon },
  { to: "/security", label: "Security", description: "Auth, TLS & encryption", icon: ShieldCheckIcon },
  { to: "/pricing", label: "Pricing", description: "Free & cloud tiers", icon: TagIcon },
  { to: "/roadmap", label: "Roadmap", description: "What's next", icon: MapIcon },
]

const developerLinks = [
  { href: "https://docs.betterkv.com", label: "Documentation", description: "Guides & API reference", icon: BookOpenIcon, external: true },
  { href: "https://docs.betterkv.com/installation", label: "Installation", description: "Get started in minutes", icon: DownloadIcon, external: true },
  { href: "https://github.com/1jmdev/BetterKV", label: "GitHub", description: "Source code & issues", icon: GitBranchIcon, external: true },
  { to: "/compare", label: "Compare", description: "BetterKV vs others", icon: ScaleIcon },
  { to: "/community", label: "Community", description: "Get involved", icon: UsersIcon },
  { to: "/changelog", label: "Changelog", description: "Release history", icon: FileTextIcon },
]

const useCaseLinks = [
  { to: "/use-cases", label: "Overview", description: "All use cases", icon: LayersIcon },
  { to: "/use-cases/caching", label: "Caching", description: "Application & API cache", icon: ZapIcon },
  { to: "/use-cases/sessions", label: "Session Storage", description: "Fast session management", icon: UserIcon },
  { to: "/use-cases/analytics", label: "Real-time Analytics", description: "Counters & leaderboards", icon: BarChart3Icon },
  { to: "/use-cases/queues", label: "Message Queues", description: "Pub/Sub & task queues", icon: MailIcon },
  { to: "/use-cases/rate-limiting", label: "Rate Limiting", description: "API throttling", icon: DatabaseIcon },
  { to: "/use-cases/feature-flags", label: "Feature Flags", description: "Toggle features instantly", icon: FlagIcon },
  { to: "/use-cases/gaming", label: "Gaming", description: "Leaderboards & state", icon: GamepadIcon },
]

function NavDropdownItem({
  item,
}: {
  item: { to?: string; href?: string; label: string; description: string; icon: React.ElementType; external?: boolean }
}) {
  const Icon = item.icon

  if (item.external && item.href) {
    return (
      <NavigationMenuLink
        href={item.href}
        target="_blank"
        rel="noopener noreferrer"
        className="flex items-start gap-3 rounded-md p-2.5 transition-colors hover:bg-accent"
      >
        <div className="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-md bg-primary/10">
          <Icon className="size-4 text-primary" />
        </div>
        <div>
          <div className="text-sm font-medium leading-tight">{item.label}</div>
          <div className="mt-0.5 text-xs text-muted-foreground">{item.description}</div>
        </div>
      </NavigationMenuLink>
    )
  }

  return (
    <NavigationMenuLink
      render={<Link to={item.to!} />}
      className="flex items-start gap-3 rounded-md p-2.5 transition-colors hover:bg-accent"
    >
      <div className="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-md bg-primary/10">
        <Icon className="size-4 text-primary" />
      </div>
      <div>
        <div className="text-sm font-medium leading-tight">{item.label}</div>
        <div className="mt-0.5 text-xs text-muted-foreground">{item.description}</div>
      </div>
    </NavigationMenuLink>
  )
}

export function Navbar() {
  return (
    <header className="sticky top-0 z-50 w-full border-b border-border/50 bg-background/80 backdrop-blur-xl">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between px-6">
        <div className="flex items-center gap-1">
          <Link to="/" className="mr-4 flex items-center gap-2.5">
            <div className="flex size-7 items-center justify-center rounded-lg bg-primary">
              <span className="text-sm font-bold text-primary-foreground">B</span>
            </div>
            <span className="text-sm font-semibold tracking-tight">BetterKV</span>
          </Link>

          <NavigationMenu>
            <NavigationMenuList>
              <NavigationMenuItem>
                <NavigationMenuTrigger>Product</NavigationMenuTrigger>
                <NavigationMenuContent className="w-[340px]">
                  <div className="grid gap-0.5 p-1.5">
                    {productLinks.map((item) => (
                      <NavDropdownItem key={item.label} item={item} />
                    ))}
                  </div>
                </NavigationMenuContent>
              </NavigationMenuItem>

              <NavigationMenuItem>
                <NavigationMenuTrigger>Developers</NavigationMenuTrigger>
                <NavigationMenuContent className="w-[340px]">
                  <div className="grid gap-0.5 p-1.5">
                    {developerLinks.map((item) => (
                      <NavDropdownItem key={item.label} item={item} />
                    ))}
                  </div>
                </NavigationMenuContent>
              </NavigationMenuItem>

              <NavigationMenuItem>
                <NavigationMenuTrigger>Use Cases</NavigationMenuTrigger>
                <NavigationMenuContent className="w-[380px]">
                  <div className="grid grid-cols-2 gap-0.5 p-1.5">
                    {useCaseLinks.map((item) => (
                      <NavigationMenuLink
                        key={item.label}
                        render={<Link to={item.to} />}
                        className="flex items-start gap-2.5 rounded-md p-2.5 transition-colors hover:bg-accent"
                      >
                        <item.icon className="mt-0.5 size-4 shrink-0 text-primary" />
                        <div>
                          <div className="text-sm font-medium leading-tight">{item.label}</div>
                          <div className="mt-0.5 text-xs text-muted-foreground">{item.description}</div>
                        </div>
                      </NavigationMenuLink>
                    ))}
                  </div>
                </NavigationMenuContent>
              </NavigationMenuItem>
            </NavigationMenuList>
          </NavigationMenu>
        </div>

        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon"
            render={<a href="https://github.com/1jmdev/BetterKV" target="_blank" rel="noopener noreferrer" aria-label="GitHub" />}
          >
            <svg viewBox="0 0 24 24" className="size-4" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
            </svg>
          </Button>
          <WaitlistModal>
            <Button size="sm" className="cursor-pointer">Join Waitlist</Button>
          </WaitlistModal>
        </div>
      </div>
    </header>
  )
}
