import React from "react";

import { BsLink45Deg } from "react-icons/bs";

import { Flex, Text, Link } from "@chakra-ui/react";
import { Project } from "../../models";
import SideMenuBlock from "../sideMenuBlock";

interface LinksBlockProps {
  project: Project;
}

const LinksBlock: React.FC<LinksBlockProps> = ({ project }) => {
  if (!project.links || project.links.length === 0) {
    return null;
  }
  return (
    <SideMenuBlock title="Links">
      <>
        {project.links.map((link) => (
          <Link isExternal href={link.url} rel="noreferrer" className="nounder">
            <Flex key={alert.name} py={1} alignItems="center">
              <BsLink45Deg />

              <Text
                px={1}
                isTruncated
                fontSize={14}
                fontWeight={500}
                title={link.name}
              >
                {link.name}
              </Text>
            </Flex>
          </Link>
        ))}
      </>
    </SideMenuBlock>
  );
};

export default LinksBlock;
