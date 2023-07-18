import { Badge, Box, Center, Flex, Text } from "@chakra-ui/react";
import { getBgColor, PrimaryOperation, valueOfApplied } from "../common";
import { OperationDiff } from "../models";

interface OperationHeaderProps {
  path: string;
  methodName: string;
  onClick: () => void;
  operation: OperationDiff;
  operationPrimary: PrimaryOperation;
}

const OperationHeader: React.FC<OperationHeaderProps> = ({
  methodName,
  path,
  onClick,
  operation,
  operationPrimary,
}) => {
  let summary =
    operation.summary && valueOfApplied(operation.summary, operationPrimary)[0];
  let summaryBgColor =
    operation.summary && getBgColor(operation.summary, operationPrimary);

  let operationId =
    operation.operationId &&
    valueOfApplied(operation.operationId, operationPrimary)[0];
  let operationIdBgColor =
    operation.operationId &&
    getBgColor(operation.operationId, operationPrimary);

  return (
    <Flex
      p={2}
      flexGrow={1}
      color="gray.50"
      onClick={onClick}
      alignItems="baseline"
    >
      <Box>
        <Badge textTransform="uppercase" width="70px">
          <Center fontSize="md">{methodName}</Center>
        </Badge>
      </Box>

      <Text fontSize="md" fontWeight={500} whiteSpace="nowrap" mx={2}>
        {path}
      </Text>

      <Flex
        flex={1}
        direction="row"
        justifyContent="space-between"
        overflow="clip"
      >
        {summary && (
          <Text
            px={1}
            isTruncated
            title={summary}
            fontSize="11px"
            borderRadius={3}
            bgColor={summaryBgColor}
          >
            {summary}
          </Text>
        )}

        {operationId && (
          <Text
            px={1}
            isTruncated
            fontSize="11px"
            borderRadius={3}
            title={operationId}
            bgColor={operationIdBgColor}
          >
            {operationId}
          </Text>
        )}
      </Flex>
    </Flex>
  );
};

export default OperationHeader;
