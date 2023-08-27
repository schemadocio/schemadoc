import React from "react";
import { Project } from "./models";
import ProjectDependencies from "./projectDependencies";

interface ProjectOverviewClientProps {
  project: Project;
}

const ProjectOverviewClient: React.FC<ProjectOverviewClientProps> = ({
  project,
}: ProjectOverviewClientProps) => {
  return <ProjectDependencies project={project} />;
};

export default ProjectOverviewClient;
