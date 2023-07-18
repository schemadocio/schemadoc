import { Box, Text } from "@chakra-ui/react";
import React, { useContext } from "react";
import { OperationDiff } from "../models";

import { DerefParameterContext } from "../http-schema";

import {
  DiffResult,
  getBgColor,
  valueOf,
  valueOfApplied,
  PrimaryOperation,
} from "../common";

import OperationResponses from "./operation-responses";
import OperationParameters from "./operation-parameters";
import OperationBody from "./operation-body";

interface OperationProps {
  colorize: boolean;
  entityPath: string;

  parentPrimary: PrimaryOperation;
  operationDiffResult: DiffResult<OperationDiff>;
}

export const Operation: React.FC<OperationProps> = ({
  entityPath,
  parentPrimary,
  operationDiffResult,
}: OperationProps) => {
  const parameterDeref = useContext(DerefParameterContext);
  if (!parameterDeref) {
    return null;
  }

  const [operation, operationPrimary] = valueOfApplied(
    operationDiffResult,
    parentPrimary
  );

  if (!operation) {
    return null;
  }

  let description = operation.description && valueOf(operation.description);
  let descriptionBgColor = getBgColor(operation.description, operationPrimary);

  return (
    <Box>
      {description && (
        <Text px={6} py={3} whiteSpace="pre-line" bgColor={descriptionBgColor}>
          {description}
        </Text>
      )}

      {operation.parameters && (
        <OperationParameters
          entityPath={entityPath}
          parentPrimary={operationPrimary}
          parametersDiffResult={operation.parameters}
        />
      )}

      {operation.requestBody && (
        <OperationBody
          entityPath={entityPath}
          parentPrimary={operationPrimary}
          mayBeRequestBodyDiffResult={operation.requestBody}
        />
      )}

      {operation.responses && (
        <OperationResponses
          entityPath={entityPath}
          parentPrimary={operationPrimary}
          mayBeResponsesDiffResultRef={operation.responses}
        />
      )}
    </Box>
  );
};

export default Operation;
