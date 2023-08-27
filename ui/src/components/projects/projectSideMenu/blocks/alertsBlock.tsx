import React from "react";

import { BsSlack, BsGoogle, BsAsterisk } from "react-icons/bs";

import { Flex, Text } from "@chakra-ui/react";
import { Project } from "../../models";
import SideMenuBlock from "../sideMenuBlock";

interface AlertsBlockProps {
  project: Project;
}

const AlertsBlock: React.FC<AlertsBlockProps> = ({ project }) => {
  if (project.alerts.length === 0) {
    return null;
  }
  return (
    <SideMenuBlock title="Alerts">
      <>
        {project.alerts.map((alert) => (
          <Flex key={alert.name} py={1} alignItems="center">
            {alert.service === "Slack" ? (
              <BsSlack
                size="12px"
                title={alert.service}
                color={alert.isActive ? "#5BB381" : "gray"}
              />
            ) : (
              <BsGoogle
                size="12px"
                title={alert.service}
                color={alert.isActive ? "#5BB381" : "gray"}
              />
            )}

            <Text
              px={1}
              isTruncated
              fontSize={14}
              fontWeight={500}
              title={alert.name}
            >
              {alert.name}
            </Text>

            {alert.kind === "breaking" && (
              <BsAsterisk
                title="Only breaking changes reported"
                color="red"
                size={8}
              />
            )}
          </Flex>
        ))}
      </>
    </SideMenuBlock>
  );
};

export default AlertsBlock;
