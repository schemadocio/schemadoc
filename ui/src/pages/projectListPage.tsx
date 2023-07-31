import React, { useEffect, useState } from "react";
import { Stack, Box, Text, Divider, Flex, Input } from "@chakra-ui/react";

import api from "../api";
import { Project } from "../components/projects/models";
import Loading from "../components/loading";
import ProjectListItem from "../components/projects/projectListItem";

interface ProjectListPageProps {}

const ProjectListPage: React.FC<ProjectListPageProps> = () => {
  const [projects, setProjects] = useState<Project[]>([]);

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [search, setSearch] = useState<string>("");

  useEffect(() => {
    setIsLoading(true);
    api.projects
      .list()
      .then((response) => setProjects(response.data))
      .finally(() => setIsLoading(false));
  }, []);

  let filteredProjects = projects.filter(
    (p) => p.slug.includes(search) || p.name.includes(search)
  );

  const children =
    filteredProjects && filteredProjects.length > 0 ? (
      filteredProjects.map((project) => (
        <ProjectListItem key={project.slug} project={project} />
      ))
    ) : (
      <Box p={2}>
        {isLoading ? (
          <Loading text="projects" />
        ) : (
          <Text>
            No projects found. Add project in configuration file or adjust
            search query
          </Text>
        )}
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
        <Flex justifyContent="space-between">
          <Text fontSize="1.3em" fontWeight="medium" pl={2}>
            Projects
          </Text>
          <Input
            autoFocus
            size="sm"
            maxWidth="480px"
            placeholder="Search projects ..."
            onChange={(e) => setSearch(e.target.value)}
          />
        </Flex>

        <Divider />
        <Box>{children}</Box>
      </Stack>
    </>
  );
};

export default ProjectListPage;
