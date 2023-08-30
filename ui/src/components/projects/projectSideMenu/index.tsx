import React from "react";

import { Link } from "react-router-dom";

import { BsList, BsDiagram2, BsArrowLeft } from "react-icons/bs";
import { Flex, Text, Box, Link as ChakraLink, VStack } from "@chakra-ui/react";

import SideMenuItem from "./sideMenuItem";
import { Project } from "../models";
import LinksBlock from "./blocks/linksBlock";
import AlertsBlock from "./blocks/alertsBlock";
import UsedByBlock from "./blocks/usedByBlock";
import DependsOnBlock from "./blocks/dependsOnBlock";
import DataSourcesBlock from "./blocks/dataSourcesBlock";

interface ProjectSideMenuProps {
  project: Project;
}

const ProjectSideMenu: React.FC<ProjectSideMenuProps> = ({ project }) => {
  return (
    <Flex
      ml={4}
      mr={2}
      top={4}
      width={260}
      maxHeight="calc(100vh - 20px)"
      position="sticky"
      flexDirection="column"
      justifyContent="space-between"
    >
      <VStack align="stretch" spacing={1}>
        <Box
          p={2}
          mb={1.5}
          cursor="pointer"
          bgColor="gray.100"
          borderRadius={5}
        >
          <Text fontSize={12} color="gray.600">
            Project
          </Text>

          <Link to="overview">
            <Text
              isTruncated
              fontSize={16}
              fontWeight={500}
              borderRadius={3}
              title={project.name}
            >
              {project.name}
            </Text>
          </Link>
        </Box>

        {project.kind === "server" && (
          <SideMenuItem icon={BsList} text="Versions" to="versions" />
        )}

        {project.kind === "server" && project.dependencies.length > 0 && (
          <SideMenuItem
            icon={BsDiagram2}
            text="Dependencies"
            to="dependencies"
          />
        )}

        <LinksBlock project={project} />
        <AlertsBlock project={project} />
        {project.kind === "server" && <DataSourcesBlock project={project} />}
        <DependsOnBlock project={project} />
        <UsedByBlock project={project} />
      </VStack>

      <Box>
        <ChakraLink
          mb={2}
          ml={2}
          as={Link}
          fontSize={18}
          className="nounder"
          to="/projects"
        >
          <Flex alignItems="center">
            <BsArrowLeft />
            <Text ml={2}>Projects</Text>
          </Flex>
        </ChakraLink>

        <Flex alignItems="center" fontSize={12} color={"gray.600"} mt={3}>
          <Text>
            Powered by{" "}
            <ChakraLink
              isExternal
              rel="noreferrer"
              className="nounder"
              href="https://github.com/schemadocio/schemadoc"
            >
              SchemaDoc
            </ChakraLink>
          </Text>
        </Flex>
      </Box>
    </Flex>
  );
};

export default ProjectSideMenu;
