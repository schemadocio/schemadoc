import React from "react";

import {
  BsDatabaseFillX,
  BsDatabaseFillDown,
  BsDatabaseFillSlash,
} from "react-icons/bs";

import { Flex, Text, Link as ChakraLink } from "@chakra-ui/react";
import { Project } from "../../models";
import SideMenuBlock from "../sideMenuBlock";
import { humanizeDateTimeOffset } from "../../../datetime";

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
            isExternal
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

export default DataSourceBlock;
