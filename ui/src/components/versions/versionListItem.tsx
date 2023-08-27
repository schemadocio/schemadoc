import React from "react";

import { Link } from "react-router-dom";
import { CgGitCommit } from "react-icons/cg";
import { Flex, Text, HStack, Icon, Link as ChakraLink } from "@chakra-ui/react";

import { Version } from "./models";
import { humanizeDateTimeOffset } from "../datetime";

interface VersionListItemProps {
  version: Version;
}

const VersionListItem: React.FC<VersionListItemProps> = ({
  version,
}: VersionListItemProps) => {
  let hover = { cursor: "pointer", textDecoration: "underline" };
  return (
    <Flex
      p={1}
      borderRadius={5}
      alignItems="center"
      justifyContent="space-between"
      _hover={{ bgColor: "gray.100" }}
    >
      <ChakraLink
        flex={1}
        as={Link}
        className="nounder"
        to={String(version.id)}
      >
        <Flex alignItems="center">
          <Icon as={CgGitCommit} mr={1} w={4} h={4} />
          <Flex flexDirection="column" alignItems="stretch" flex={1}>
            <Text>
              [{version.id}] {version.message}
            </Text>

            <Text color="gray.400" fontSize={14}>
              {version.version || "none"}
            </Text>
          </Flex>
        </Flex>
      </ChakraLink>

      <HStack mx={5} spacing={5}>
        <Text color="gray.400" fontStyle="italic" mr={1}>
          {humanizeDateTimeOffset(version.createdAt)}
        </Text>

        <HStack spacing={10} width="240px" justifyContent="space-between">
          <Link to={`${version.id}?diffTypeFilters=added`}>
            <Text color="green.600" _hover={hover}>
              +{version.statistics.added}
            </Text>
          </Link>
          <Link to={`${version.id}?diffTypeFilters=updated`}>
            <Text color="orange.600" _hover={hover}>
              {version.statistics.updated}
            </Text>
          </Link>
          <Link to={`${version.id}?diffTypeFilters=removed`}>
            <Text color="red.800" _hover={hover}>
              -{version.statistics.removed}
            </Text>
          </Link>
          <Link to={`${version.id}?diffTypeFilters=all`}>
            <Text color="gray.700" _hover={hover}>
              {version.statistics.total}
            </Text>
          </Link>
        </HStack>
      </HStack>
    </Flex>
  );
};

export default VersionListItem;
