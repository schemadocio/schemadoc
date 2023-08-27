import React, { useEffect, useState } from "react";

import { Box, Text } from "@chakra-ui/react";

import { useLocation } from "react-router-dom";
import { Project } from "./models";
import { Version } from "../versions/models";
import api from "../../api";
import { humanizeDateTimeOffset } from "../datetime";
import VersionView from "../versions/versionView";

interface ProjectOverviewServerProps {
  project: Project;
}

const ProjectOverviewServer: React.FC<ProjectOverviewServerProps> = ({
  project,
}: ProjectOverviewServerProps) => {
  const { hash } = useLocation();

  const [version, setVersion] = useState<Version | null>(null);

  useEffect(() => {
    if (project.branches.length === 0) {
      return;
    }

    api.versions.list(project.slug, project.branches[0]).then(({ data }) => {
      if (data.length > 0) {
        setVersion(data[0]);
      }
    });
  }, [project.slug, project.branches]);

  if (project.branches.length === 0) {
    return <Box p={3}>No branches found for the project</Box>;
  }

  if (!version) {
    return <Box p={3}>No versions found for the project</Box>;
  }

  return (
    <Box pr={3} flex={1} display="flex" borderRadius={5} flexDirection="column">
      <Box
        p={3}
        height="58px"
        bgColor="gray.50"
        maxWidth={1120}
        borderRadius={5}
        display="flex"
        width="100%"
        alignItems="center"
        justifyContent="space-between"
      >
        <Box>
          <Text display="inline" color="gray.600">
            Latest update:
          </Text>{" "}
          [{version.id}]{" "}
          {version.message || (
            <Text as="span" color="gray.500" fontStyle="italic">
              No message
            </Text>
          )}
        </Box>
        <Text display="inline" color="gray.400" fontStyle="italic">
          {humanizeDateTimeOffset(version.createdAt)}
        </Text>
      </Box>

      <VersionView
        project={project}
        versionId={version.id}
        branchName={project.branches[0]}
        options={{ focusPath: decodeURI(hash) }}
      />
    </Box>
  );
};

export default ProjectOverviewServer;
