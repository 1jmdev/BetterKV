export interface DocNavItem {
  title: string;
  href: string;
}

export interface DocNavSection {
  title: string;
  items: DocNavItem[];
}

export const docsNav: DocNavSection[] = [
  {
    title: "Getting Started",
    items: [
      { title: "Introduction", href: "/docs" },
      { title: "Installation", href: "/docs/installation" },
      { title: "Quickstart", href: "/docs/quickstart" },
    ],
  },
  {
    title: "Guides",
    items: [
      { title: "Configuration", href: "/docs/configuration" },
      { title: "CLI Reference", href: "/docs/cli" },
    ],
  },
  {
    title: "Reference",
    items: [
      { title: "Data Types", href: "/docs/data-types" },
      { title: "Redis Compatibility", href: "/docs/compatibility" },
    ],
  },
];

export function getDocByHref(href: string): DocNavItem | undefined {
  for (const section of docsNav) {
    const item = section.items.find((i) => i.href === href);
    if (item) return item;
  }
  return undefined;
}

export function getAdjacentDocs(href: string): {
  prev: DocNavItem | undefined;
  next: DocNavItem | undefined;
} {
  const allItems = docsNav.flatMap((s) => s.items);
  const index = allItems.findIndex((i) => i.href === href);
  return {
    prev: index > 0 ? allItems[index - 1] : undefined,
    next: index < allItems.length - 1 ? allItems[index + 1] : undefined,
  };
}
