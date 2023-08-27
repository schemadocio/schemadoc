import React, { useContext } from "react";

import ProjectContext from "../components/projects/projectContext";

import ProjectDependencies from "../components/projects/projectDependencies";

interface VersionListPageProps {}

const DependenciesPage: React.FC<VersionListPageProps> = () => {
  const project = useContext(ProjectContext);

  if (!project) {
    return null;
  }

  return <ProjectDependencies project={project} />;
};

export default DependenciesPage;
