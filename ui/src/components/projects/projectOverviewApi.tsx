import React, { useEffect, useState } from "react";

import { Box, Text } from "@chakra-ui/react";

import { useLocation } from "react-router-dom";
import { Project } from "./models";
import { Version } from "../versions/models";
import api from "../../api";
import { humanizeDateTimeOffset } from "../datetime";
import VersionView from "../versions/versionView";

interface ProjectOverviewAPIProps {
  project: Project;
}

const ProjectOverviewAPI: React.FC<ProjectOverviewAPIProps> = ({
  project,
}: ProjectOverviewAPIProps) => {
  const { hash } = useLocation();

  const [version, setVersion] = useState<Version | null>(null);

  useEffect(() => {
    api.versions.list(project.slug).then(({ data }) => {
      if (data.length > 0) {
        setVersion(data[0]);
      } else {
        setVersion(null);
      }
    });
  }, [project]);

  let versionComponent = null;
  if (!version) {
    versionComponent = <Box p={3}>No versions found for the project</Box>;
  } else {
    versionComponent = (
      <>
        <Box
          p={3}
          bgColor="gray.50"
          maxWidth={1120}
          borderRadius={5}
          display="flex"
          width="100%"
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
          options={{ focusPath: decodeURI(hash) }}
        />
      </>
    );
  }

  return (
    <Box pr={3} flex={1} display="flex" borderRadius={5} flexDirection="column">
      {versionComponent}
    </Box>
  );
};

export default ProjectOverviewAPI;
