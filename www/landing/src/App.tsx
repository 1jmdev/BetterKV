import { BrowserRouter, Routes, Route } from "react-router-dom"
import { Layout } from "@/components/layout/Layout"
import { ScrollToTop } from "@/components/layout/ScrollToTop"
import { LandingPage } from "@/pages/Landing"
import { PerformancePage } from "@/pages/Performance"
import { CompatibilityPage } from "@/pages/Compatibility"
import { RoadmapPage } from "@/pages/Roadmap"
import { ChangelogPage } from "@/pages/Changelog"
import { PricingPage } from "@/pages/Pricing"
import { AboutPage } from "@/pages/About"
import { SecurityPage } from "@/pages/Security"
import { UseCasesOverviewPage } from "@/pages/use-cases/UseCasesOverview"
import { CachingPage } from "@/pages/use-cases/Caching"
import { SessionsPage } from "@/pages/use-cases/Sessions"
import { AnalyticsPage } from "@/pages/use-cases/Analytics"
import { QueuesPage } from "@/pages/use-cases/Queues"
import { RateLimitingPage } from "@/pages/use-cases/RateLimiting"
import { FeatureFlagsPage } from "@/pages/use-cases/FeatureFlags"
import { GamingPage } from "@/pages/use-cases/Gaming"
import { CommunityPage } from "@/pages/Community"
import { ComparePage } from "@/pages/Compare"
import { PrivacyPage } from "@/pages/legal/Privacy"
import { TermsPage } from "@/pages/legal/Terms"

export default function App() {
  return (
    <BrowserRouter>
      <ScrollToTop />
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<LandingPage />} />
          <Route path="/performance" element={<PerformancePage />} />
          <Route path="/compatibility" element={<CompatibilityPage />} />
          <Route path="/roadmap" element={<RoadmapPage />} />
          <Route path="/changelog" element={<ChangelogPage />} />
          <Route path="/pricing" element={<PricingPage />} />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/security" element={<SecurityPage />} />
          <Route path="/use-cases" element={<UseCasesOverviewPage />} />
          <Route path="/use-cases/caching" element={<CachingPage />} />
          <Route path="/use-cases/sessions" element={<SessionsPage />} />
          <Route path="/use-cases/analytics" element={<AnalyticsPage />} />
          <Route path="/use-cases/queues" element={<QueuesPage />} />
          <Route path="/use-cases/rate-limiting" element={<RateLimitingPage />} />
          <Route path="/use-cases/feature-flags" element={<FeatureFlagsPage />} />
          <Route path="/use-cases/gaming" element={<GamingPage />} />
          <Route path="/community" element={<CommunityPage />} />
          <Route path="/compare" element={<ComparePage />} />
          <Route path="/privacy" element={<PrivacyPage />} />
          <Route path="/terms" element={<TermsPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}
