import { useContext } from "react";
import ProjectContext from "../components/projects/projectContext";
import ProjectOverviewServer from "../components/projects/projectOverviewServer";
import ProjectOverviewClient from "../components/projects/projectOverviewClient";

interface ProjectOverviewPageProps {}

const ProjectOverviewPage: React.FC<ProjectOverviewPageProps> = () => {
  const project = useContext(ProjectContext);
  if (!project) {
    return null;
  }

  if (project.kind === "server") {
    return <ProjectOverviewServer key={project.slug} project={project} />;
  }
  return <ProjectOverviewClient key={project.slug} project={project} />;
};

export default ProjectOverviewPage;
