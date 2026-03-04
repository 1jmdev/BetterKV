import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { CheckIcon } from "lucide-react"

export function WaitlistModal({ children }: { children: React.ReactNode }) {
  const [email, setEmail] = useState("")
  const [submitted, setSubmitted] = useState(false)
  const [open, setOpen] = useState(false)

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    if (!email) return
    setSubmitted(true)
    setEmail("")
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(nextOpen) => {
        setOpen(nextOpen)
        if (!nextOpen) {
          setTimeout(() => setSubmitted(false), 300)
        }
      }}
    >
      <DialogTrigger
        onClick={() => setOpen(true)}
        render={<span className="inline-flex" />}
      >
        {children}
      </DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>
            {submitted ? "You're on the list" : "Join the Waitlist"}
          </DialogTitle>
          <DialogDescription>
            {submitted
              ? "We'll notify you when BetterKV Cloud is ready."
              : "Get early access to BetterKV Cloud — managed, hosted, and ready to scale."}
          </DialogDescription>
        </DialogHeader>
        {submitted ? (
          <div className="flex items-center gap-3 rounded-lg bg-primary/10 p-4">
            <div className="flex size-8 shrink-0 items-center justify-center rounded-full bg-primary/20">
              <CheckIcon className="size-4 text-primary" />
            </div>
            <p className="text-sm text-muted-foreground">
              We'll send updates to your inbox. No spam, ever.
            </p>
          </div>
        ) : (
          <form onSubmit={handleSubmit} className="flex gap-2">
            <Input
              type="email"
              placeholder="you@example.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              className="flex-1"
            />
            <Button type="submit">Subscribe</Button>
          </form>
        )}
      </DialogContent>
    </Dialog>
  )
}
