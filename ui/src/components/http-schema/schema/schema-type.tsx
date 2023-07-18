import { Box, Flex, Text } from "@chakra-ui/react";
import React, { useContext } from "react";

import {
  getBgColor,
  derefValue,
  EitherDiffIs,
  valueOfApplied,
  PrimaryOperation,
  VecDiff,
  DiffResult,
  valueOfChange,
} from "../common";
import { SchemaDiff } from "../models";
import { DerefSchemaContext } from "../http-schema";

interface SchemaTypeProps {
  schema: SchemaDiff;
  schemaPrimary: PrimaryOperation;
  topmost?: boolean;
  wrapped?: boolean;
}

export const SchemaType: React.FC<SchemaTypeProps> = ({
  schema,
  schemaPrimary,
  topmost = true,
  wrapped = true,
}) => {
  // TODO: can be null
  if (!schema.type) {
    return null;
  }

  const [innerType, innerTypeSchema] = valueOfApplied(
    schema.type,
    schemaPrimary
  );
  if (!innerType) {
    return null;
  }

  let component = null;

  switch (innerType.t) {
    case EitherDiffIs.Left:
      component = (
        <SchemaTypeLeft
          wrapped={wrapped}
          topmost={topmost}
          schema={schema}
          schemaPrimary={schemaPrimary}
          innerType={innerType.v}
        />
      );
      break;
    case EitherDiffIs.Right:
      component = (
        <SchemaTypeRight
          innerType={innerType.v}
          innerTypeSchema={innerTypeSchema}
        />
      );
      break;
  }

  return component;
};

export default SchemaType;

interface SchemaTypeLeftProps {
  innerType: DiffResult<string>;
  schema: SchemaDiff;
  schemaPrimary: PrimaryOperation;
  topmost?: boolean;
  wrapped?: boolean;
}

const SchemaTypeLeft: React.FC<SchemaTypeLeftProps> = ({
  schema,
  schemaPrimary,

  innerType,

  topmost,
  wrapped,
}) => {
  const [oldType, sameType, newType] = valueOfChange(innerType, schemaPrimary);

  if (!oldType && !sameType && !newType) {
    return null;
  }

  const [oldFormat, sameFormat, newFormat] = schema.format
    ? valueOfChange(schema.format, schemaPrimary)
    : [null, null, null];

  if (sameType && oldFormat === newFormat) {
    let component = (
      <SchemaTypeLeftValue
        schema={schema}
        schemaPrimary={schemaPrimary}
        type={sameType}
        topmost={topmost}
        format={sameFormat}
      />
    );
    return (
      <Flex align="baseline" lineHeight="normal" color={"blue.700"}>
        {wrapped ? (
          <Box borderRadius={3} p={0.5}>
            {component}
          </Box>
        ) : (
          component
        )}
      </Flex>
    );
  }

  const typeChanged =
    oldType !== null && newType !== null && oldType !== newType;

  const formatChanged =
    oldFormat !== null && newFormat !== null && oldFormat !== newFormat;

  const changed = typeChanged || formatChanged;

  let oldBgColor = "transparent";
  let newBgColor = "transparent";
  if (changed) {
    oldBgColor = "red.100";
    newBgColor = "green.100";
  }

  let oldComponent = null;
  if (oldType) {
    oldComponent = (
      <SchemaTypeLeftValue
        schema={schema}
        schemaPrimary={schemaPrimary}
        type={oldType}
        topmost={topmost}
        format={oldFormat}
      />
    );
  }

  let newComponent = null;
  if (newType) {
    newComponent = (
      <SchemaTypeLeftValue
        schema={schema}
        schemaPrimary={schemaPrimary}
        type={newType}
        topmost={topmost}
        format={newFormat}
      />
    );
  }

  return (
    <Flex align="baseline" lineHeight="normal" color={"blue.700"}>
      {oldComponent &&
        (wrapped ? (
          <Box bgColor={oldBgColor} borderRadius={3} p={0.5}>
            {oldComponent}
          </Box>
        ) : (
          oldComponent
        ))}

      {changed ? (
        <Text as="span" mr={1} ml={1}>
          →
        </Text>
      ) : null}

      {newComponent &&
        (wrapped ? (
          <Box bgColor={newBgColor} borderRadius={3} p={0.5}>
            {newComponent}
          </Box>
        ) : (
          newComponent
        ))}
    </Flex>
  );
};

interface SchemaTypeLeftValueProps {
  schema: SchemaDiff;
  schemaPrimary: PrimaryOperation;
  topmost?: boolean;
  type: string;
  format: string | null;
}

const SchemaTypeLeftValue: React.FC<SchemaTypeLeftValueProps> = ({
  topmost,
  type,
  format,
  schema,
  schemaPrimary,
}) => {
  const schemaDeref = useContext(DerefSchemaContext);
  if (!schemaDeref) {
    return null;
  }

  const formatComponent = format && (
    <Text as="span" color="gray.600">
      (${format})
    </Text>
  );

  if (type === "array" && !topmost) {
    let itemsSchema = null;
    let itemsSchemaPrimary = null;
    if (schema.items) {
      const [mayBeRefSchema, mayBeRefSchemaPrimary] = valueOfApplied(
        schema.items,
        schemaPrimary
      );
      if (mayBeRefSchema) {
        const schemaEntity = derefValue(mayBeRefSchema, schemaDeref);
        if (schemaEntity) {
          [itemsSchema, itemsSchemaPrimary] = valueOfApplied(
            schemaEntity,
            mayBeRefSchemaPrimary
          );
        }
      }
    }

    if (!itemsSchema || !itemsSchemaPrimary) {
      return null;
    }

    return (
      <Flex alignItems="baseline">
        [
        <SchemaType
          wrapped={false}
          schema={itemsSchema}
          schemaPrimary={itemsSchemaPrimary}
        />
        ]{formatComponent}
      </Flex>
    );
  }

  return (
    <>
      {type}
      {formatComponent}
    </>
  );
};

interface SchemaTypeRightProps {
  innerType: DiffResult<VecDiff<string>>;
  innerTypeSchema: PrimaryOperation;
}

const SchemaTypeRight: React.FC<SchemaTypeRightProps> = ({
  innerType,
  innerTypeSchema,
}) => {
  let [typeVec, typeVecSchema] = valueOfApplied(innerType, innerTypeSchema);
  if (!typeVec) {
    return null;
  }

  let texts = typeVec.map((valueDiffResult) => {
    const bgColor = getBgColor(valueDiffResult, typeVecSchema);
    const [value, _] = valueOfApplied(valueDiffResult, typeVecSchema);

    return (
      <Text
        py={1}
        key={value}
        color={"blue.700"}
        display="inline-block"
        backgroundColor={bgColor}
      >
        {value}
      </Text>
    );
  });

  return (
    <>
      {texts.reduce((prev: React.ReactElement[], curr, idx) => {
        if (prev.length) {
          prev.push(<span key={`${idx}-or`}> | </span>);
        }
        prev.push(curr);
        return prev;
      }, [])}
    </>
  );
};
