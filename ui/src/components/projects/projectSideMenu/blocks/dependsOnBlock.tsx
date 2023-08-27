import React from "react";

import { Link } from "react-router-dom";
import { BsDot, BsArrowUpRight } from "react-icons/bs";
import { Flex, Text, Link as ChakraLink } from "@chakra-ui/react";

import { Project } from "../../models";
import SideMenuBlock from "../sideMenuBlock";

interface DependsOnBlockProps {
  project: Project;
}

const DependsOnBlock: React.FC<DependsOnBlockProps> = ({ project }) => {
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
            <Flex
              my={1}
              px={1}
              borderRadius={5}
              bgColor={bgColor}
              alignItems="center"
              key={dependency.project}
              justifyContent={"space-between"}
            >
              <ChakraLink
                as={Link}
                className="nounder"
                to={`dependencies?dep=${dependency.project}`}
              >
                <Flex alignItems="center">
                  <BsDot />
                  <Text
                    px={1}
                    as="span"
                    isTruncated
                    fontSize={14}
                    fontWeight={500}
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
              <ChakraLink
                as={Link}
                className="nounder"
                title="Go to the project page"
                to={`/projects/${dependency.project}`}
              >
                <BsArrowUpRight />
              </ChakraLink>
            </Flex>
          );
        })}
      </>
    </SideMenuBlock>
  );
};

export default DependsOnBlock;
