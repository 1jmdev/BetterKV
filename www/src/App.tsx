import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Layout } from "@/components/layout/Layout";
import { DocsLayout } from "@/components/layout/DocsLayout";
import { LandingPage } from "@/pages/Landing";
import { BenchmarksPage } from "@/pages/Benchmarks";
import { DocsIntroduction } from "@/pages/docs/Introduction";
import { DocsInstallation } from "@/pages/docs/Installation";
import { DocsQuickstart } from "@/pages/docs/Quickstart";
import { DocsConfiguration } from "@/pages/docs/Configuration";
import { DocsCli } from "@/pages/docs/Cli";
import { DocsDataTypes } from "@/pages/docs/DataTypes";
import { DocsCompatibility } from "@/pages/docs/Compatibility";
import { ScrollToTop } from "@/components/scroll-to-top";

export default function App() {
  return (
    <BrowserRouter>
      <ScrollToTop />
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<LandingPage />} />
          <Route path="/benchmarks" element={<BenchmarksPage />} />
          <Route path="/docs" element={<DocsLayout />}>
            <Route index element={<DocsIntroduction />} />
            <Route path="installation" element={<DocsInstallation />} />
            <Route path="quickstart" element={<DocsQuickstart />} />
            <Route path="configuration" element={<DocsConfiguration />} />
            <Route path="cli" element={<DocsCli />} />
            <Route path="data-types" element={<DocsDataTypes />} />
            <Route path="compatibility" element={<DocsCompatibility />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
