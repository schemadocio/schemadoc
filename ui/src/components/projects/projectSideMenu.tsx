import React, { useEffect, useState } from "react";

import {
  BsList,
  BsLink45Deg,
  BsSlack,
  BsGoogle,
  BsAsterisk,
  BsDatabaseFillX,
  BsDatabaseFillDown,
  BsDatabaseFillSlash,
} from "react-icons/bs";

import {
  Flex,
  Text,
  Icon,
  Box,
  Link as ChakraLink,
  VStack,
} from "@chakra-ui/react";
import { Link, NavLink as RouterLink } from "react-router-dom";

import { Dependency, Project } from "./models";
import { humanizeDateTimeOffset } from "../datetime";

import api from "../../api";
import Loading from "../loading";

interface ProjectSideMenuProps {
  project: Project;
}

const ProjectSideMenu: React.FC<ProjectSideMenuProps> = ({ project }) => {
  return (
    <Flex
      ml={4}
      mr={2}
      top={4}
      width={240}
      maxHeight="calc(100vh - 20px)"
      position="sticky"
      flexDirection="column"
      justifyContent="space-between"
    >
      <VStack align="stretch" spacing={1}>
        <Link to="overview">
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
            <Text
              isTruncated
              _hover={{ color: "#d0165a" }}
              fontSize={16}
              fontWeight={500}
              borderRadius={3}
              title={project.name}
            >
              {project.name}
            </Text>
          </Box>
        </Link>

        {project.kind === "server" && (
          <SideMenuItem icon={BsList} text="Versions" to="versions" />
        )}

        {project.links &&
          project.links.map((link) => (
            <SideMenuItem
              icon={BsLink45Deg}
              text={link.name}
              to={link.url}
              target="_blank"
            />
          ))}

        <AlertsBlock project={project} />
        {project.kind === "server" && <DataSourceBlock project={project} />}
        <DependenciesBlock project={project} />
        <DependentBlock project={project} />
      </VStack>

      <Link to="/projects">
        <Text mb={2} fontSize="3xl" fontWeight="300" display="inline-block">
          <Text as="span" color="#d0165a">
            Schema
          </Text>
          <Text as="span" fontWeight="400">
            Doc
          </Text>
        </Text>
      </Link>
    </Flex>
  );
};

interface DependenciesBlockProps {
  project: Project;
}

const DependenciesBlock: React.FC<DependenciesBlockProps> = ({ project }) => {
  if (!project.dependencies || project.dependencies.length === 0) {
    return null;
  }

  return (
    <SideMenuBlock title="Depends on">
      <>
        {project.dependencies.map((dependency) => {
          const bgColor = dependency.breaking
            ? "red.200"
            : dependency.outdated
            ? "orange.200"
            : "transparent";

          return (
            <ChakraLink
              as={Link}
              key={dependency.project}
              to={`/projects/${dependency.project}`}
              _hover={{ textDecoration: "none", color: "#d0165a" }}
            >
              <Flex
                my={1}
                px={1}
                borderRadius={5}
                bgColor={bgColor}
                alignItems="center"
                key={dependency.project}
              >
                -
                <Text
                  px={1}
                  as="span"
                  isTruncated
                  fontSize={14}
                  fontWeight={500}
                  textAlign="center"
                  title={dependency.project}
                >
                  {dependency.project}
                </Text>
                <Text
                  as="span"
                  isTruncated
                  fontSize={12}
                  title={dependency.project}
                >
                  [{dependency.version}]
                </Text>
              </Flex>
            </ChakraLink>
          );
        })}
      </>
    </SideMenuBlock>
  );
};

interface DependentBlockProps {
  project: Project;
}

const DependentBlock: React.FC<DependentBlockProps> = ({ project }) => {
  const [dependents, setDependents] = useState<Dependency[] | null>(null);

  useEffect(() => {
    api.projects.dependents(project.slug).then(({ data }) => {
      setDependents(data);
    });
  }, [project.slug]);

  if (dependents === null) {
    return <Loading />;
  }

  if (dependents.length === 0) {
    return null;
  }

  return (
    <SideMenuBlock title="Used by">
      <>
        {dependents.map((dependency) => {
          const bgColor = dependency.breaking
            ? "red.200"
            : dependency.outdated
            ? "orange.200"
            : "transparent";

          return (
            <ChakraLink
              as={Link}
              key={dependency.project}
              to={`/projects/${dependency.project}`}
              _hover={{ textDecoration: "none", color: "#d0165a" }}
            >
              <Flex
                my={1}
                px={1}
                borderRadius={5}
                bgColor={bgColor}
                alignItems="center"
                key={dependency.project}
              >
                -
                <Text
                  px={1}
                  as="span"
                  isTruncated
                  fontSize={14}
                  fontWeight={500}
                  textAlign="center"
                  title={dependency.project}
                >
                  {dependency.project}
                </Text>
                <Text
                  as="span"
                  isTruncated
                  fontSize={12}
                  title={dependency.project}
                >
                  [{dependency.version}]
                </Text>
              </Flex>
            </ChakraLink>
          );
        })}
      </>
    </SideMenuBlock>
  );
};

interface AlertsBlockProps {
  project: Project;
}

const AlertsBlock: React.FC<AlertsBlockProps> = ({ project }) => {
  return (
    project.alerts && (
      <SideMenuBlock title="Alerts">
        <>
          {project.alerts.map((alert) => (
            <Flex key={alert.name} py={1} alignItems="center">
              {alert.service === "Slack" ? (
                <BsSlack
                  size="14px"
                  title={alert.service}
                  color={alert.isActive ? "#5BB381" : "gray"}
                />
              ) : (
                <BsGoogle
                  size="14px"
                  title={alert.service}
                  color={alert.isActive ? "#5BB381" : "gray"}
                />
              )}

              <Text
                px={1}
                isTruncated
                fontSize={14}
                fontWeight={500}
                title={alert.name}
              >
                {alert.name}
              </Text>

              {alert.kind === "breaking" && (
                <BsAsterisk
                  title="Only breaking changes reported"
                  color="red"
                  size={8}
                />
              )}
            </Flex>
          ))}
        </>
      </SideMenuBlock>
    )
  );
};

interface DataSourceBlockProps {
  project: Project;
}

const DataSourceBlock: React.FC<DataSourceBlockProps> = ({ project }) => {
  let ds = project.dataSource;
  if (!ds) {
    return null;
  }

  let icon = (
    <BsDatabaseFillDown size="14px" color="#5BB381" title="Pull Enabled" />
  );
  if (ds.status) {
    if (ds.status.pullError) {
      icon = <BsDatabaseFillX size="14px" color="red" title="Pull error" />;
    } else if (!ds.status.pullEnabled) {
      icon = (
        <BsDatabaseFillSlash size="14px" color="gray" title="Pull disabled" />
      );
    }
  }

  return (
    <SideMenuBlock title="Data Source">
      <Flex direction="column">
        <Flex key={alert.name} py={1} alignItems="center">
          {icon}
          <ChakraLink
            href={ds.source.Url?.url}
            target="_blank"
            referrerPolicy="strict-origin"
          >
            <Text
              pl={1}
              isTruncated
              fontSize={14}
              fontWeight={500}
              title={ds.source.Url?.url}
            >
              {ds.name}
            </Text>
          </ChakraLink>
        </Flex>
        {ds.status && (
          <>
            <Text ml="18px" fontSize={12} color="gray.600">
              <Text as="span" fontWeight="medium">
                pulled:
              </Text>{" "}
              {humanizeDateTimeOffset(ds.status.pullLastAt)}
            </Text>

            <Text ml="18px" fontSize={12} color="gray.600">
              <Text as="span" fontWeight="medium">
                pull interval:
              </Text>{" "}
              {ds.status.pullIntervalMinutes} minutes
            </Text>
          </>
        )}
      </Flex>
    </SideMenuBlock>
  );
};

interface SideMenuBlockProps {
  title: string;
  children?: React.ReactElement | null;
}

const SideMenuBlock: React.FC<SideMenuBlockProps> = ({ title, children }) => (
  <Box p={2} borderRadius={5}>
    <Text fontSize={12} color="gray.600" fontWeight="medium">
      {title}
    </Text>
    {children}
  </Box>
);

interface SideMenuItemProps {
  text: string;
  icon: any;
  to: string;
  target?: string;
}

const SideMenuItem: React.FC<SideMenuItemProps> = ({
  target,
  text,
  icon,
  to,
}: SideMenuItemProps) => {
  return (
    <RouterLink
      to={to}
      target={target}
      className={({ isActive }) => (isActive ? "active" : "inactive")}
    >
      {({ isActive }) => (
        <Flex
          padding={2}
          borderRadius="5px"
          alignItems="center"
          // bgColor={isActive ? "#cbd5e0b8" : undefined}
          bgColor={isActive ? "gray.300" : undefined}
          color={isActive ? "#d0165a" : undefined}
          style={{ textDecoration: "none" }}
          _focus={{ boxShadow: "none" }}
          _hover={{ bgColor: "gray.300", color: "#d0165a" }}
        >
          <Icon as={icon} mr={3} w={6} h={6} />
          <Text>{text}</Text>
        </Flex>
      )}
    </RouterLink>
  );
};

export default ProjectSideMenu;
