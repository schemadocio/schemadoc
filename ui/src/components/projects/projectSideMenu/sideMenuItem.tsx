import React from "react";

import { Flex, Text, Icon } from "@chakra-ui/react";
import { NavLink as RouterLink } from "react-router-dom";

interface SideMenuItemProps {
  text: string;
  icon: any;
  to: string;
  target?: string;
}

const SideMenuItem: React.FC<SideMenuItemProps> = ({
  target,
  text,
  icon,
  to,
}: SideMenuItemProps) => {
  return (
    <RouterLink
      to={to}
      target={target}
      className={({ isActive }) => (isActive ? "active" : "inactive")}
    >
      {({ isActive }) => (
        <Flex
          padding={2}
          borderRadius="5px"
          alignItems="center"
          // bgColor={isActive ? "#cbd5e0b8" : undefined}
          bgColor={isActive ? "gray.300" : undefined}
          color={isActive ? "#d0165a" : undefined}
          style={{ textDecoration: "none" }}
          _focus={{ boxShadow: "none" }}
          _hover={{ bgColor: "gray.300", color: "#d0165a" }}
        >
          <Icon as={icon} mr={3} w={6} h={6} />
          <Text>{text}</Text>
        </Flex>
      )}
    </RouterLink>
  );
};

export default SideMenuItem;
