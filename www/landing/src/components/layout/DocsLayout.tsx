import { Outlet } from "react-router-dom";
import { DocsSidebar } from "./DocsSidebar";

export function DocsLayout() {
  return (
    <div className="mx-auto max-w-6xl px-4 sm:px-6">
      <div className="flex gap-8 py-8 lg:py-12">
        <DocsSidebar />
        <article className="min-w-0 flex-1">
          <Outlet />
        </article>
      </div>
    </div>
  );
}
