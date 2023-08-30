import React, { useEffect, useState } from "react";
import { Box, HStack } from "@chakra-ui/react";
import { Outlet, useParams } from "react-router-dom";

import api from "../../api";
import { Project } from "./models";
import ProjectContext from "./projectContext";
import ProjectSideMenu from "./projectSideMenu";

interface ProjectLayoutProps {}

const ProjectLayout: React.FC<ProjectLayoutProps> = ({}) => {
  const { projectSlug } = useParams();

  const [project, setProject] = useState<Project | null>(null);

  useEffect(() => {
    if (projectSlug) {
      api.projects.get(projectSlug).then(({ data }) => setProject(data.result));
    }
  }, [projectSlug]);

  if (!project) {
    return null;
  }

  return (
    <HStack flex="1" alignItems="stretch" mt={4}>
      <ProjectSideMenu project={project} />
      <Box width="100%" display="flex" flex={1}>
        <ProjectContext.Provider value={project}>
          <Outlet />
        </ProjectContext.Provider>
      </Box>
    </HStack>
  );
};

export default ProjectLayout;
