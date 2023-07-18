import React from "react";

import { Link } from "react-router-dom";
import { CgGitCommit } from "react-icons/cg";
import { Flex, Text, HStack, Icon } from "@chakra-ui/react";

import { Version } from "./models";
import { humanizeDateTimeOffset } from "../datetime";

interface VersionListItemProps {
  version: Version;
}

const VersionListItem: React.FC<VersionListItemProps> = ({
  version,
}: VersionListItemProps) => {
  return (
    <Link to={String(version.id)}>
      <HStack p={1} borderRadius={5} _hover={{ bgColor: "gray.100" }}>
        <Icon as={CgGitCommit} mr={1} w={4} h={4} />
        <Flex flexDirection="column" alignItems="stretch" flex={1}>
          <Text>
            [{version.id}] {version.message}
          </Text>

          <Flex
            justifyContent="space-between"
            alignItems="baseline"
            color="gray.600"
            fontSize={14}
            flex={1}
          >
            <Text>{version.version || "none"}</Text>
            <Text fontStyle="italic" mr={1}>
              {humanizeDateTimeOffset(version.createdAt)}
            </Text>
          </Flex>
        </Flex>
      </HStack>
    </Link>
  );
};

export default VersionListItem;
