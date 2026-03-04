import { Badge } from "@/components/ui/badge"

interface PageHeaderProps {
  badge?: string
  title: string
  description: string
}

export function PageHeader({ badge, title, description }: PageHeaderProps) {
  return (
    <div className="border-b border-border/50 bg-background">
      <div className="mx-auto max-w-6xl px-6 py-20">
        {badge && (
          <Badge variant="secondary" className="mb-4">
            {badge}
          </Badge>
        )}
        <h1 className="max-w-2xl text-4xl font-bold tracking-tight">{title}</h1>
        <p className="mt-4 max-w-xl text-lg text-muted-foreground">{description}</p>
      </div>
    </div>
  )
}
