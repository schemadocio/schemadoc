import { Routes, Route, Outlet, Navigate } from "react-router-dom";
import AppLayout from "./AppLayout";
import ProjectLayout from "./components/projects/projectLayout";
import VersionsComparePage from "./pages/versionsComparePage";
import ProjectListPage from "./pages/projectListPage";
import ProjectOverviewPage from "./pages/projectOverviewPage";
import VersionPage from "./pages/versionPage";
import VersionListPage from "./pages/versionListPage";
import DependenciesPage from "./pages/dependenciesPage";

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
            <Route path="dependencies" element={<DependenciesPage />} />

            <Route path="versions/:branchName?" element={<VersionListPage />} />
            <Route
              path="versions/:branchName/:versionId"
              element={<VersionPage />}
            />
            <Route
              path="versions/:sourceBranch/:sourceId/compare/:targetBranch/:targetId"
              element={<VersionsComparePage />}
            />
          </Route>
        </Route>
      </Route>
    </Routes>
  );
}

export default Router;
