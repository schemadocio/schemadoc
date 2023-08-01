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
      <Flex
        p={1}
        borderRadius={5}
        alignItems="center"
        _hover={{ bgColor: "gray.100" }}
        justifyContent="space-between"
      >
        <Icon as={CgGitCommit} mr={1} w={4} h={4} />
        <Flex flexDirection="column" alignItems="stretch" flex={1}>
          <Text>
            [{version.id}] {version.message}
          </Text>

          <Text color="gray.400" fontSize={14}>
            {version.version || "none"}
          </Text>
        </Flex>

        <HStack
          mx={5}
          flex={1}
          spacing={5}
          alignItems="baseline"
          justifyContent="right"
        >
          <Text color="gray.400" fontStyle="italic" mr={1}>
            {humanizeDateTimeOffset(version.createdAt)}
          </Text>

          <HStack spacing={10}>
            <Link to={`${version.id}?diffTypeFilters=added`}>
              <Text
                color="green.600"
                _hover={{ cursor: "pointer", textDecoration: "underline" }}
              >
                +{version.statistics.added}
              </Text>
            </Link>
            <Link to={`${version.id}?diffTypeFilters=updated`}>
              <Text
                color="orange.600"
                _hover={{ cursor: "pointer", textDecoration: "underline" }}
              >
                {version.statistics.updated}
              </Text>
            </Link>
            <Link to={`${version.id}?diffTypeFilters=removed`}>
              <Text
                color="red.800"
                _hover={{ cursor: "pointer", textDecoration: "underline" }}
              >
                -{version.statistics.removed}
              </Text>
            </Link>
            <Link to={`${version.id}?diffTypeFilters=all`}>
              <Text
                color="gray.700"
                _hover={{ cursor: "pointer", textDecoration: "underline" }}
              >
                {version.statistics.total}
              </Text>
            </Link>
          </HStack>
        </HStack>
      </Flex>
    </Link>
  );
};

export default VersionListItem;
