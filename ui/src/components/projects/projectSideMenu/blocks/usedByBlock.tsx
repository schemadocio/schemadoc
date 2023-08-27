import React, { useEffect, useState } from "react";

import { BsDot } from "react-icons/bs";
import { Link } from "react-router-dom";
import { Flex, Text, Link as ChakraLink } from "@chakra-ui/react";

import api from "../../../../api";
import Loading from "../../../loading";
import SideMenuBlock from "../sideMenuBlock";
import { Dependency, Project } from "../../models";

interface UsedByBlockProps {
  project: Project;
}

const UsedByBlock: React.FC<UsedByBlockProps> = ({ project }) => {
  const [dependents, setDependents] = useState<Dependency[] | null>(null);

  useEffect(() => {
    api.projects.dependents(project.slug).then(({ data }) => {
      setDependents(data);
    });
  }, [project.slug]);

  if (dependents === null) {
    return <Loading />;
  }

  if (dependents.length === 0) {
    return null;
  }

  return (
    <SideMenuBlock title="Used by">
      <>
        {dependents.map((dependency) => {
          const bgColor = dependency.breaking
            ? "red.200"
            : dependency.outdated
            ? "orange.200"
            : "transparent";

          return (
            <ChakraLink
              as={Link}
              className="nounder"
              key={dependency.project}
              to={`/projects/${dependency.project}/dependencies?dep=${project.slug}`}
            >
              <Flex
                my={1}
                px={1}
                borderRadius={5}
                bgColor={bgColor}
                alignItems="center"
                key={dependency.project}
              >
                <BsDot />
                <Text
                  px={1}
                  as="span"
                  isTruncated
                  fontSize={14}
                  fontWeight={500}
                  textAlign="center"
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
          );
        })}
      </>
    </SideMenuBlock>
  );
};

export default UsedByBlock;
