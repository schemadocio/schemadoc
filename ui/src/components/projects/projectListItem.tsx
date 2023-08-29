import React from "react";
import { Link } from "react-router-dom";
import { Box, Text, Flex } from "@chakra-ui/react";

import { Project } from "./models";

interface ProjectListItemProps {
  project: Project;
}

const ProjectListItem: React.FC<ProjectListItemProps> = ({
  project,
}: ProjectListItemProps) => {
  return (
    <Link to={project.slug}>
      <Flex
        p={3}
        borderRadius={5}
        cursor="pointer"
        alignItems="center"
        justifyContent="space-between"
        _hover={{ bgColor: "gray.100" }}
      >
        <Box>
          <Text fontSize="1.1em" fontWeight={500}>
            {project.name}
          </Text>
          <Text fontSize="0.8em" fontWeight="400" color="gray.700">
            {project.slug}
          </Text>
          <Text fontSize="0.8em" color="gray.700">
            {project.description}
          </Text>
        </Box>
      </Flex>
    </Link>
  );
};

export default ProjectListItem;
