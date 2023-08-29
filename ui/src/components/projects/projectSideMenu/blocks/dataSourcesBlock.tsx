import React from "react";

import {
  BsArrowUpRight,
  BsDatabaseFillX,
  BsDatabaseFillDown,
  BsDatabaseFillSlash,
} from "react-icons/bs";

import {
  Flex,
  Text,
  Link as ChakraLink,
  useDisclosure,
  Collapse,
} from "@chakra-ui/react";
import { DataSource, Project } from "../../models";
import SideMenuBlock from "../sideMenuBlock";
import { humanizeDateTimeOffset } from "../../../datetime";

interface DataSourceBlockProps {
  project: Project;
}

const DataSourcesBlock: React.FC<DataSourceBlockProps> = ({ project }) => {
  if (project.dataSources.length === 0) {
    return null;
  }

  return (
    <SideMenuBlock title="Data Sources">
      <>
        {project.dataSources.map((datasource) => (
          <DataSourceRow datasource={datasource} key={datasource.name} />
        ))}
      </>
    </SideMenuBlock>
  );
};

interface DataSourceRowProps {
  datasource: DataSource;
}
const DataSourceRow: React.FC<DataSourceRowProps> = ({ datasource }) => {
  const { isOpen, onToggle } = useDisclosure();

  let icon = (
    <BsDatabaseFillDown size="14px" color="#5BB381" title="Pull Enabled" />
  );
  if (datasource.status) {
    if (datasource.status.pullError) {
      icon = <BsDatabaseFillX size="14px" color="red" title="Pull error" />;
    } else if (!datasource.status.pullEnabled) {
      icon = (
        <BsDatabaseFillSlash size="14px" color="gray" title="Pull disabled" />
      );
    }
  }
  return (
    <Flex direction="column">
      <Flex
        py={1}
        key={alert.name}
        alignItems="center"
        justifyContent="space-between"
      >
        <Flex alignItems="center" ml={1}>
          {icon}
          <ChakraLink onClick={onToggle} className="nounder">
            <Text
              pl={1}
              isTruncated
              fontSize={14}
              fontWeight={500}
              maxWidth="220px"
              title={`${datasource.name} [${datasource.branch}]`}
            >
              {datasource.name} [{datasource.branch}]
            </Text>
          </ChakraLink>
        </Flex>

        <ChakraLink
          isExternal
          referrerPolicy="strict-origin"
          href={datasource.source.Url?.url}
          title={datasource.source.Url?.url}
        >
          <BsArrowUpRight />
        </ChakraLink>
      </Flex>

      <Collapse in={isOpen} animateOpacity={false}>
        <Flex direction="column">
          <Text ml="18px" fontSize={12} color="gray.600">
            <Text as="span" fontWeight="medium">
              branch:
            </Text>{" "}
            {datasource.branch}
          </Text>

          {datasource.status && (
            <>
              <Text ml="18px" fontSize={12} color="gray.600">
                <Text as="span" fontWeight="medium">
                  pulled:
                </Text>{" "}
                {humanizeDateTimeOffset(datasource.status.pullLastAt)}
              </Text>

              <Text ml="18px" fontSize={12} color="gray.600">
                <Text as="span" fontWeight="medium">
                  pull interval:
                </Text>{" "}
                {datasource.status.pullIntervalMinutes} minutes
              </Text>
            </>
          )}
        </Flex>
      </Collapse>
    </Flex>
  );
};

export default DataSourcesBlock;
