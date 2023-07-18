import React, { useContext } from "react";

import { Text, Box } from "@chakra-ui/react";

import { DerefSchemaContext } from "../http-schema";
import { SchemaDiff } from "../models";

import {
  valueOfApplied,
  DiffResult,
  MayBeRefDiff,
  EitherDiff,
  EitherDiffIs,
  PrimaryOperation,
  getBgColor,
  valueOf,
} from "../common";
import { Schema } from "./schema";

interface SchemaAdditionalPropertiesProps {
  entityPath: string;
  renderDepth: number;
  parentPrimary: PrimaryOperation;
  eitherAdditionalPropertiesDiffResult: DiffResult<
    EitherDiff<boolean, MayBeRefDiff<SchemaDiff>>
  >;
}

export const SchemaAdditionalProperties: React.FC<
  SchemaAdditionalPropertiesProps
> = ({
  entityPath,
  renderDepth,
  parentPrimary,
  eitherAdditionalPropertiesDiffResult,
}: SchemaAdditionalPropertiesProps) => {
  const derefSchema = useContext(DerefSchemaContext);
  if (!derefSchema) {
    return null;
  }

  const [eitherAdditionalProperties, _] = valueOfApplied(
    eitherAdditionalPropertiesDiffResult,
    parentPrimary
  );

  if (!eitherAdditionalProperties) {
    return null;
  }

  if (
    eitherAdditionalProperties.t === EitherDiffIs.Left ||
    eitherAdditionalProperties.t === EitherDiffIs.ToLeft
  ) {
    return (
      <SchemaAdditionalPropertiesLeft
        value={eitherAdditionalProperties.v}
        parentPrimary={parentPrimary}
      />
    );
  }

  return (
    <SchemaAdditionalPropertiesRight
      entityPath={entityPath}
      renderDepth={renderDepth}
      value={eitherAdditionalProperties.v}
      parentPrimary={parentPrimary}
    />
  );
};

export default SchemaAdditionalProperties;

interface SchemaAdditionalPropertiesLeftProps {
  value: DiffResult<boolean>;
  parentPrimary: PrimaryOperation;
}

const SchemaAdditionalPropertiesLeft: React.FC<
  SchemaAdditionalPropertiesLeftProps
> = ({ value, parentPrimary }) => {
  const backgroundColor = getBgColor(value, parentPrimary);

  const text = valueOf(value) ? "allowed" : "not allowed";

  return (
    <Box
      pt={1}
      pb={1}
      fontSize={12}
      display="flex"
      flexDirection="row"
      bgColor={backgroundColor}
    >
      <Text pl={2} minWidth={150} title="Additional fields" color="gray.600">
        Additional properties are{" "}
        <Text as="span" fontWeight="medium" m={0} pr={2}>
          {text}
        </Text>
      </Text>
    </Box>
  );
};

interface SchemaAdditionalPropertiesRightProps {
  value: DiffResult<MayBeRefDiff<SchemaDiff>>;
  parentPrimary: PrimaryOperation;

  entityPath: string;
  renderDepth: number;
}

const SchemaAdditionalPropertiesRight: React.FC<
  SchemaAdditionalPropertiesRightProps
> = ({ value, parentPrimary, entityPath, renderDepth }) => {
  const backgroundColor = getBgColor(value, parentPrimary);

  return (
    <Box
      pt={1}
      pb={1}
      pr={1}
      fontSize={12}
      display="flex"
      flexDirection="row"
      alignItems="baseline"
      bgColor={backgroundColor}
    >
      <Text
        pl={3}
        minWidth={150}
        fontSize={12}
        fontWeight="medium"
        title="Additional fields"
      >
        Additional properties
      </Text>
      <Schema
        entityPath={entityPath}
        renderDepth={renderDepth + 1}
        parentPrimary={parentPrimary}
        mayBeSchemaDiffResult={value}
      />
    </Box>
  );
};
