import React from "react";
import { Outlet } from "react-router-dom";
import { Flex } from "@chakra-ui/react";

interface AppLayoutProps {}

const AppLayout: React.FC<AppLayoutProps> = () => {
  return (
    <Flex
      minHeight="100vh"
      bgColor="gray.200"
      flexDirection="column"
      justifyContent="space-between"
    >
      <Flex flexGrow={1} flexDirection="column">
        {/* <Flex
          p={3}
          height="55px"
          bgColor="gray.50"
          alignItems="center"
          borderBottom="solid 1px gray"
          justifyContent="space-between"
        >
          <Link to="projects">
            <Text fontSize="3xl" fontWeight="300" display="inline-block">
              Schema
              <Text as="span" fontWeight="400">
                Doc
              </Text>
            </Text>
          </Link>
        </Flex> */}
        <Outlet />
      </Flex>
    </Flex>
  );
};

export default AppLayout;
