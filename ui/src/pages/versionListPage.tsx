import React, { useContext, useEffect, useState } from "react";
import { Flex, Text, Stack, useDisclosure, Button } from "@chakra-ui/react";

import ProjectContext from "../components/projects/projectContext";
import { Project } from "../components/projects/models";
import { Version } from "../components/versions/models";

import api from "../api";
import VersionListItem from "../components/versions/versionListItem";
import VersionCompareModal from "../components/modals/versionsCompareModal";

interface VersionListPageProps {}

const VersionListPage: React.FC<VersionListPageProps> = () => {
  const project = useContext(ProjectContext);

  if (!project) {
    return null;
  }

  return <VersionList project={project} />;
};

export default VersionListPage;

interface VersionListProps {
  project: Project;
}

export const VersionList: React.FC<VersionListProps> = ({ project }) => {
  const [versions, setVersions] = useState<Version[] | null>(null);

  useEffect(() => {
    api.versions.list(project.slug).then(({ data }) => setVersions(data));
  }, [project.slug]);

  const {
    isOpen: compareVersionsIsOpen,
    onOpen: compareVersionsOnOpen,
    onClose: compareVersionsOnClose,
  } = useDisclosure();

  return (
    <Flex flex="1">
      <Flex maxWidth="1120px" width="100%" flexDirection="column">
        <Flex justifyContent="space-between">
          <Text fontSize="2xl">Versions</Text>
          <Button size="sm" colorScheme="green" onClick={compareVersionsOnOpen}>
            Compare versions
          </Button>
        </Flex>
        <Stack
          mt={2}
          padding={2}
          width="100%"
          spacing="6px"
          borderRadius={5}
          bgColor="gray.50"
        >
          {versions && versions.length > 0 ? (
            versions.map((version) => (
              <VersionListItem key={version.id} version={version} />
            ))
          ) : (
            <div>No data</div>
          )}
        </Stack>
      </Flex>

      {compareVersionsIsOpen && (
        <VersionCompareModal
          project={project}
          isOpen={compareVersionsIsOpen}
          onClose={compareVersionsOnClose}
        />
      )}
    </Flex>
  );
};
