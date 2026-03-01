import { useState } from "react";
import { Check, Copy } from "lucide-react";
import { cn } from "@/lib/utils";
import Prism from "prismjs";
import "prismjs/themes/prism-tomorrow.css";
import "prismjs/components/prism-bash";
import "prismjs/components/prism-typescript";
import "prismjs/components/prism-javascript";
import "prismjs/components/prism-python";
import "prismjs/components/prism-json";
import "prismjs/components/prism-css";
import "prismjs/components/prism-markup";

interface CodeBlockProps {
  code: string;
  language?: string;
  title?: string;
  className?: string;
  showLineNumbers?: boolean;
}

function highlight(code: string, language: string): string {
  const grammar = Prism.languages[language] ?? Prism.languages.plain;
  return Prism.highlight(code, grammar, language);
}

export function CodeBlock({
  code,
  language = "bash",
  title,
  className,
  showLineNumbers = false,
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const highlighted = highlight(code, language);
  const lines = highlighted.split("\n");

  return (
    <div
      className={cn(
        "group relative rounded-lg border border-border bg-[oklch(0.15_0.005_285)] overflow-hidden",
        className
      )}
    >
      {title && (
        <div className="flex items-center justify-between border-b border-border px-4 py-2">
          <span className="text-xs font-medium text-muted-foreground">
            {title}
          </span>
          <span className="text-xs text-muted-foreground/60">{language}</span>
        </div>
      )}
      <div className="relative">
        <pre className="overflow-x-auto p-4 text-sm leading-relaxed !bg-transparent">
          <code className="font-mono">
            {showLineNumbers
              ? lines.map((line, i) => (
                  <span key={i} className="table-row">
                    <span className="table-cell select-none pr-4 text-right text-muted-foreground/40 text-xs">
                      {i + 1}
                    </span>
                    <span
                      className="table-cell"
                      dangerouslySetInnerHTML={{ __html: line }}
                    />
                    {i < lines.length - 1 && "\n"}
                  </span>
                ))
              : <span dangerouslySetInnerHTML={{ __html: highlighted }} />}
          </code>
        </pre>
        <button
          onClick={handleCopy}
          className="absolute right-2 top-2 rounded-md border border-border bg-background/80 p-1.5 opacity-0 backdrop-blur-sm transition-all hover:bg-accent group-hover:opacity-100"
          aria-label="Copy code"
        >
          {copied ? (
            <Check className="size-3.5 text-green-400" />
          ) : (
            <Copy className="size-3.5 text-muted-foreground" />
          )}
        </button>
      </div>
    </div>
  );
}

interface InlineCodeProps {
  children: React.ReactNode;
  className?: string;
}

export function InlineCode({ children, className }: InlineCodeProps) {
  return (
    <code
      className={cn(
        "rounded-md bg-muted/60 px-1.5 py-0.5 font-mono text-sm text-primary",
        className
      )}
    >
      {children}
    </code>
  );
}
