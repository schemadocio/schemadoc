import React from "react";

import { Text, Box } from "@chakra-ui/react";

interface SideMenuBlockProps {
  title: string;
  children?: React.ReactElement | null;
}

const SideMenuBlock: React.FC<SideMenuBlockProps> = ({ title, children }) => (
  <Box p={1} borderRadius={5}>
    <Text fontSize={12} color="gray.600" fontWeight="medium">
      {title}
    </Text>
    {children}
  </Box>
);

export default SideMenuBlock;
