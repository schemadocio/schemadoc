import React, { useContext, useEffect, useState } from "react";
import {
  Flex,
  Text,
  Stack,
  useDisclosure,
  Button,
  Box,
  Select,
} from "@chakra-ui/react";

import { useNavigate, useParams } from "react-router-dom";

import ProjectContext from "../components/projects/projectContext";
import { Project } from "../components/projects/models";
import { Version } from "../components/versions/models";

import api from "../api";
import VersionListItem from "../components/versions/versionListItem";
import VersionCompareModal from "../components/modals/versionsCompareModal";

interface VersionListPageProps {}

const VersionListPage: React.FC<VersionListPageProps> = () => {
  const { branchName } = useParams();

  const navigate = useNavigate();

  const project = useContext(ProjectContext);

  useEffect(() => {
    if (!branchName && project) {
      navigate(project.branches[0]);
    }
  }, [branchName, project, navigate]);

  if (!project) {
    return null;
  }

  if (!project.branches[0] || !branchName) {
    return <Box>No branches found for the project</Box>;
  }

  return <VersionList project={project} branchName={branchName} />;
};

export default VersionListPage;

interface VersionListProps {
  project: Project;
  branchName: string;
}

export const VersionList: React.FC<VersionListProps> = ({
  project,
  branchName,
}) => {
  const navigate = useNavigate();

  const [versions, setVersions] = useState<Version[] | null>(null);

  useEffect(() => {
    api.versions
      .list(project.slug, branchName)
      .then(({ data }) => setVersions(data));
  }, [project.slug, branchName]);

  const {
    isOpen: compareVersionsIsOpen,
    onOpen: compareVersionsOnOpen,
    onClose: compareVersionsOnClose,
  } = useDisclosure();

  return (
    <Flex flex="1">
      <Flex maxWidth="1120px" width="100%" flexDirection="column">
        <Flex justifyContent="space-between">
          <Flex>
            <Text fontSize="2xl">Versions</Text>
            <Select
              ml={1}
              onChange={(e) =>
                navigate(`../${e.target.value}`, { relative: "path" })
              }
            >
              {project.branches.map((branch) => (
                <option
                  key={branch}
                  value={branch}
                  selected={branch === branchName}
                >
                  {branch}
                </option>
              ))}
            </Select>
          </Flex>
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
            <Box mx={2}>No data</Box>
          )}
        </Stack>
      </Flex>

      {compareVersionsIsOpen && (
        <VersionCompareModal
          project={project}
          defaultSourceBranch={branchName}
          isOpen={compareVersionsIsOpen}
          onClose={compareVersionsOnClose}
        />
      )}
    </Flex>
  );
};
