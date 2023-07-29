import React, { useContext } from "react";

import { Box, HStack, VStack } from "@chakra-ui/react";

import { DerefSchemaContext } from "../http-schema";
import { SchemaDiff } from "../models";

import {
  valueOfApplied,
  DiffResult,
  getBgColor,
  MayBeRefDiff,
  VecDiff,
  PrimaryOperation,
} from "../common";
import Schema from ".";

interface VecPropertyProps {
  name: string;
  entityPath: string;
  renderDepth: number;
  parentPrimary: PrimaryOperation;
  mayBeSchemaVecDiffResult: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>;
}

export const VecProperty: React.FC<VecPropertyProps> = ({
  name,
  entityPath,
  renderDepth,
  parentPrimary,
  mayBeSchemaVecDiffResult,
}: VecPropertyProps) => {
  const deref = useContext(DerefSchemaContext);
  if (!deref) {
    return null;
  }

  const containerBG = getBgColor(mayBeSchemaVecDiffResult, parentPrimary);

  const [mayBeSchemaVec, mayBeSchemaVecPrimary] = valueOfApplied(
    mayBeSchemaVecDiffResult,
    parentPrimary
  );
  if (!mayBeSchemaVec) {
    return null;
  }

  const schemas = mayBeSchemaVec.map((mayBeSchemaDiffResult, idx) => {
    const backgroundColor = getBgColor(
      mayBeSchemaDiffResult,
      mayBeSchemaVecPrimary
    );

    return (
      <Box key={idx} p={1} bgColor={backgroundColor} borderRadius={3}>
        <Schema
          open={false}
          schemaName={`${idx}`}
          renderDepth={renderDepth + 1}
          entityPath={`${entityPath}/${idx}`}
          parentPrimary={mayBeSchemaVecPrimary}
          mayBeSchemaDiffResult={mayBeSchemaDiffResult}
        />
      </Box>
    );
  });

  return (
    <HStack
      pt={1}
      pb={1}
      pr={1}
      id={entityPath}
      borderRadius={3}
      align="flex-start"
      backgroundColor={containerBG}
    >
      <Box fontWeight={500} minWidth="70px" pl={3} fontSize={14}>
        {name}
      </Box>
      <VStack alignItems="flex-start" pl={1} spacing={1}>
        {schemas}
      </VStack>
    </HStack>
  );
};
