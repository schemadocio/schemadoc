import React, { useContext } from "react";

import { Box, Flex, Link, Text } from "@chakra-ui/react";

import { DerefSchemaContext } from "../http-schema";
import { SchemaDiff } from "../models";

import {
  valueOf,
  valueOfApplied,
  derefValue,
  DiffResult,
  getBgColor,
  MayBeRefDiff,
  getConcreteTypes,
  DiffResultIs,
  PrimaryOperation,
} from "../common";

import Schema from "./schema";
import SchemaType from "./schema-type";
import SchemaAttribute from "./schema-attribute";
import SchemaAttributes from "./schema-attributes";
import RequiredBadge from "../components/required-badge";

const REQUIRED_DEFAULT: DiffResult<string> = {
  t: DiffResultIs.None,
  v: null,
};

interface PropertyProps {
  name: string;
  entityPath: string;
  required?: DiffResult<string>;
  renderDepth: number;
  propertiesPrimary: PrimaryOperation;
  mayBePropertyDiffResult: DiffResult<MayBeRefDiff<SchemaDiff>>;
}

export const Property: React.FC<PropertyProps> = ({
  name,
  required = REQUIRED_DEFAULT,
  entityPath,
  renderDepth,
  propertiesPrimary,
  mayBePropertyDiffResult,
}: PropertyProps) => {
  const deref = useContext(DerefSchemaContext);
  if (!deref) {
    return null;
  }

  const backgroundColor = getBgColor(
    mayBePropertyDiffResult,
    propertiesPrimary
  );

  const [mayBeSchema, mayBeSchemaPrimary] = valueOfApplied(
    mayBePropertyDiffResult,
    propertiesPrimary
  );
  if (!mayBeSchema) {
    return null;
  }

  const schemaDiffResult = derefValue(mayBeSchema, deref);
  if (!schemaDiffResult) {
    return null;
  }

  const [schemaDiff, schemaDiffPrimary] = valueOfApplied(
    schemaDiffResult,
    mayBeSchemaPrimary
  );
  if (!schemaDiff || !schemaDiffPrimary) {
    return null;
  }

  const customFields =
    schemaDiff.customFields && valueOf(schemaDiff.customFields);

  const propertyTypes =
    (schemaDiff.type && getConcreteTypes(schemaDiff.type)) || [];

  const isObject =
    !!schemaDiff.not ||
    !!schemaDiff.oneOf ||
    !!schemaDiff.allOf ||
    !!schemaDiff.anyOf ||
    propertyTypes.some((pt: string) => pt === "object");

  return (
    <Flex
      py={1}
      pl={2}
      pr={0.5}
      mb="1px"
      id={entityPath}
      fontSize={12}
      align="flex-start"
      borderRadius={3}
      flexDirection={"column"}
      backgroundColor={backgroundColor}
    >
      <Flex fontWeight="medium" alignItems="baseline">
        <Link href={entityPath} minWidth={148} mr={1} display="flex">
          <Text as="span" title={name} isTruncated verticalAlign="middle">
            {name}
          </Text>
          <RequiredBadge required={required} primary={propertiesPrimary} />
        </Link>

        {propertyTypes.length ? (
          <Box fontWeight="bolder">
            <SchemaType schema={schemaDiff} schemaPrimary={schemaDiffPrimary} />
          </Box>
        ) : (
          <Box fontWeight="bolder" color={"blue.700"}>
            {/* object? */}
          </Box>
        )}
      </Flex>

      <Flex
        pt={0.5}
        px={1}
        flex={1}
        ml={148}
        align="left"
        fontSize={12}
        flexDirection="column"
      >
        <SchemaAttributes
          schemaDiff={schemaDiff}
          parentPrimary={schemaDiffPrimary}
          exclude={isObject ? ["example"] : []}
        />

        {(isObject || schemaDiff.items) && (
          <Box ml={0.5} mt={0.5}>
            {isObject && (
              <Schema
                fieldName={name}
                excludeAttributes={["description"]}
                entityPath={`${entityPath}/schema`}
                parentPrimary={propertiesPrimary}
                mayBeSchemaDiffResult={mayBePropertyDiffResult}
                renderDepth={renderDepth + 1}
              />
            )}

            {schemaDiff.items && (
              <Schema
                schemaName={name.toUpperCase()}
                entityPath={`${entityPath}/items`}
                parentPrimary={schemaDiffPrimary}
                mayBeSchemaDiffResult={schemaDiff.items}
                renderDepth={renderDepth + 1}
              />
            )}
          </Box>
        )}

        {customFields && (
          <Box>
            {Object.entries(customFields).map(([field, value]) => (
              <SchemaAttribute
                key={field}
                name={field}
                attributeDiffResult={value}
                parentPrimary={schemaDiffPrimary}
              />
            ))}
          </Box>
        )}
      </Flex>
    </Flex>
  );
};
