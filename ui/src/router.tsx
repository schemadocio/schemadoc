import { Routes, Route, Outlet, Navigate } from "react-router-dom";
import AppLayout from "./AppLayout";
import ProjectLayout from "./components/projects/projectLayout";
import CompareVersionsPage from "./pages/compareVersionsPage";
import ProjectListPage from "./pages/projectListPage";
import ProjectOverviewPage from "./pages/projectOverviewPage";
import VersionPage from "./pages/versionPage";
import VersionListPage from "./pages/versionListPage";

function Router() {
  return (
    <Routes>
      <Route path="/" element={<AppLayout />}>
        <Route index element={<Navigate replace to="projects" />} />

        <Route path="projects" element={<Outlet />}>
          <Route index element={<ProjectListPage />} />
          <Route path=":projectSlug" element={<ProjectLayout />}>
            <Route index element={<ProjectOverviewPage />} />
            <Route path="overview" element={<ProjectOverviewPage />} />
            <Route path="versions" element={<VersionListPage />} />
            <Route path="versions/:versionId" element={<VersionPage />} />
            <Route
              path="versions/:sourceId/compare/:targetId"
              element={<CompareVersionsPage />}
            />
          </Route>
        </Route>
      </Route>
    </Routes>
  );
}

export default Router;
