import React, { useEffect, useState } from "react";
import { Stack, Box, Text, Divider } from "@chakra-ui/react";

import api from "../api";
import { Project } from "../components/projects/models";
import Loading from "../components/loading";
import ProjectListItem from "../components/projects/projectListItem";

interface ProjectListPageProps {}

const ProjectListPage: React.FC<ProjectListPageProps> = () => {
  const [projects, setProjects] = useState<Project[]>([]);

  const [isLoading, setIsLoading] = useState<boolean>(false);

  useEffect(() => {
    setIsLoading(true);
    api.projects
      .list()
      .then((response) => setProjects(response.data))
      .finally(() => setIsLoading(false));
  }, []);

  const children =
    projects && projects.length > 0 ? (
      projects.map((project) => (
        <ProjectListItem key={project.slug} project={project} />
      ))
    ) : (
      <Box p={2}>
        {isLoading ? <Loading text="projects" /> : <Text>No data</Text>}
      </Box>
    );

  return (
    <>
      <Stack
        p={3}
        my={5}
        mx="auto"
        spacing={3}
        width="100%"
        borderRadius={5}
        maxWidth={1120}
        bgColor="gray.50"
      >
        <Text fontSize="1.3em" fontWeight="medium" pl={2}>
          Projects
        </Text>
        <Divider />
        <Box>{children}</Box>
      </Stack>
    </>
  );
};

export default ProjectListPage;
