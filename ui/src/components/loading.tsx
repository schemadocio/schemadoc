import { Box, Spinner, Text } from "@chakra-ui/react";

interface LoadingProps {
  text?: string;
  maxWidth?: number | string;
}

const Loading: React.FC<LoadingProps> = ({
  text = "",
  maxWidth = "1120px",
}) => {
  return (
    <Box
      flex={1}
      display="flex"
      maxWidth={maxWidth}
      alignItems="center"
      justifyContent="center"
      flexDirection="column"
    >
      <Spinner m={3} size="xl" />
      {text && <Text>Loading {text} ...</Text>}
    </Box>
  );
};

export default Loading;
