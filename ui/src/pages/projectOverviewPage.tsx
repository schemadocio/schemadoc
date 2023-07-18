import { useContext } from "react";
import ProjectContext from "../components/projects/projectContext";
import ProjectOverviewAPI from "../components/projects/projectOverviewApi";

interface ProjectOverviewPageProps {}

const ProjectOverviewPage: React.FC<ProjectOverviewPageProps> = () => {
  const project = useContext(ProjectContext);
  if (!project) {
    return null;
  }

  return <ProjectOverviewAPI project={project} />;
};

export default ProjectOverviewPage;
