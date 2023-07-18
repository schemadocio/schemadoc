import React, { useContext, useState } from "react";

import { Box, Text, Link } from "@chakra-ui/react";

import { DerefSchemaContext, FocusPathContext } from "../http-schema";
import { SchemaDiff } from "../models";

import {
  getKey,
  valueOf,
  derefValue,
  DiffResult,
  MayBeRefDiff,
  valueOfApplied,
  getConcreteTypes,
  PrimaryOperation,
  getBgColor,
} from "../common";

import { Property } from "./property";
import { VecProperty } from "./vec-property";
import { SchemaType } from "./schema-type";
import SchemaAdditionalProperties from "./schema-additional-properties";
import SchemaAttributes from "./schema-attributes";

interface RequiredMap extends Record<string, DiffResult<string>> {}

interface SchemaDiffProps {
  open?: boolean;
  overflow?: string;

  fieldName?: string;
  schemaName?: string;

  renderDepth: number;

  width?: string | number;

  excludeAttributes?: string[];

  parentPrimary: PrimaryOperation;

  entityPath: string;
  mayBeSchemaDiffResult: DiffResult<MayBeRefDiff<SchemaDiff>>;
}

export const Schema: React.FC<SchemaDiffProps> = ({
  open = false,
  width = "fit-content",
  overflow,
  entityPath,
  fieldName,
  schemaName,
  renderDepth,
  parentPrimary,
  excludeAttributes = [],
  mayBeSchemaDiffResult,
}: SchemaDiffProps) => {
  const focusPath = useContext(FocusPathContext);

  const isExpandedDefault =
    open || (focusPath !== null && focusPath.startsWith(entityPath));

  const [showExample, setShowExample] = useState<boolean>(false);
  const [isExpanded, setIsExpanded] = useState<boolean>(isExpandedDefault);

  const deref = useContext(DerefSchemaContext);
  if (!deref) {
    return null;
  }

  const [mayBeSchema, mayBeSchemaPrimary] = valueOfApplied(
    mayBeSchemaDiffResult,
    parentPrimary
  );
  if (!mayBeSchema || !mayBeSchemaPrimary) {
    return null;
  }

  const schemaDiffResult = derefValue(mayBeSchema, deref);
  if (!schemaDiffResult) {
    return null;
  }

  const derefferredSchemaName = getKey(mayBeSchema, deref);

  let [schemaDiff, schemaDiffPrimary] = valueOfApplied(
    schemaDiffResult,
    mayBeSchemaPrimary
  );
  if (!schemaDiff || !schemaDiffPrimary) {
    return null;
  }

  let bgColor = getBgColor(schemaDiffResult, parentPrimary);

  let schemaTitle =
    (schemaDiff.title && valueOf(schemaDiff.title)) ||
    derefferredSchemaName ||
    schemaName ||
    fieldName;

  schemaTitle = schemaTitle?.split("_").join(" ");

  const schemaTitleBgColor = getBgColor(schemaDiff.title, schemaDiffPrimary);

  const schemaMinWidth = renderDepth === 0 ? "480px" : "0px";

  let requiredMap: RequiredMap = {};
  if (schemaDiff.required) {
    let required = valueOf(schemaDiff.required);
    if (required) {
      for (let index = 0; index < required.length; index++) {
        const element = required[index];
        let requiredValue = valueOf(element);
        if (requiredValue) {
          requiredMap[requiredValue] = element;
        }
      }
    }
  }

  const schemaTypes =
    (schemaDiff.type && getConcreteTypes(schemaDiff.type)) || [];

  const isArray = schemaTypes.some((sc) => sc === "array");
  // If schema is a root schema
  if (isArray && schemaDiff.items) {
    return (
      <Box
        p={2}
        width={width}
        bgColor={bgColor}
        fontSize="14px"
        lineHeight="normal"
        borderRadius={3}
        overflow={overflow}
        marginRight="auto"
        minWidth={schemaMinWidth}
        bg="rgba(192, 192, 192, 0.3)"
      >
        <Text>array[</Text>
        <Box px={3} py={1.5}>
          <Schema
            open={open}
            width={width}
            renderDepth={renderDepth + 1}
            parentPrimary={schemaDiffPrimary}
            entityPath={`${entityPath}/items`}
            mayBeSchemaDiffResult={schemaDiff.items}
          />
        </Box>
        <Text>]</Text>
      </Box>
    );
  }

  const isObject = schemaTypes.some((sc) => sc === "object");

  // render schema for simple type
  if (!isObject && schemaTypes.length) {
    return (
      <Box width={width} bgColor={bgColor} borderRadius={3}>
        <Box
          p={1}
          width={width}
          borderRadius={3}
          overflow={overflow}
          marginRight="auto"
          lineHeight="normal"
          minWidth={schemaMinWidth}
          bg="rgba(192, 192, 192, 0.3)"
        >
          <Text as="span" fontWeight="medium" color={"blue.700"}>
            <SchemaType schema={schemaDiff} schemaPrimary={schemaDiffPrimary} />
          </Text>
          <Box fontSize={12}>
            <SchemaAttributes
              schemaDiff={schemaDiff}
              parentPrimary={schemaDiffPrimary}
            />
          </Box>
        </Box>
      </Box>
    );
  }

  const example = schemaDiff.example && (
    <Text
      p={2}
      mb={1}
      color="white"
      borderRadius={3}
      whiteSpace="break-spaces"
      backgroundColor="#3d3d3d"
    >
      {JSON.stringify(valueOf(schemaDiff.example), null, 2)}
    </Text>
  );

  const isCompound =
    schemaDiff.not || schemaDiff.oneOf || schemaDiff.allOf || schemaDiff.anyOf;

  let properties = null;
  let propertiesPrimary = null;
  if (schemaDiff.properties) {
    [properties, propertiesPrimary] = valueOfApplied(
      schemaDiff.properties,
      schemaDiffPrimary
    );
  }

  const showEmptyBrackets =
    isObject &&
    !isCompound &&
    !showExample &&
    !schemaDiff.additionalProperties &&
    Object.keys(properties || {}).length === 0;

  return (
    <Box width={width} bgColor={bgColor} borderRadius={3}>
      <Box
        p={2}
        width={width}
        borderRadius={3}
        overflow={overflow}
        lineHeight="normal"
        minWidth={schemaMinWidth}
        bgColor={"rgba(192, 192, 192, 0.3)"}
      >
        <>
          <Text
            fontSize={15}
            fontWeight={500}
            lineHeight="normal"
            id={`${entityPath}/title`}
          >
            <Text
              as="span"
              px={1}
              ml={-1}
              borderRadius={3}
              title={schemaTitle}
              textTransform="capitalize"
              bgColor={schemaTitleBgColor}
            >
              {schemaTitle}
            </Text>
            <Link ml={1} onClick={() => setIsExpanded(!isExpanded)}>
              {isExpanded ? "↑" : "↓"}
            </Link>
          </Text>

          {isExpanded && (
            <Box pt={1.5}>
              <Link
                fontSize={12}
                onClick={() => setShowExample(false)}
                textDecoration={showExample ? undefined : "underline"}
              >
                Schema
              </Link>
              <> | </>
              <Link
                fontSize={12}
                onClick={() => setShowExample(true)}
                textDecoration={showExample ? "underline" : undefined}
              >
                Examples
              </Link>
            </Box>
          )}
        </>

        {isExpanded && (
          <Box pt={1}>
            {showExample && example}

            {showEmptyBrackets && (
              <Text fontSize={14} pt={1}>
                {"{ }"}
              </Text>
            )}

            <Box fontSize={12} py={1}>
              <SchemaAttributes
                schemaDiff={schemaDiff}
                parentPrimary={schemaDiffPrimary}
                exclude={[...excludeAttributes, "example", "title"]}
              />
            </Box>

            {!showExample &&
              properties &&
              Object.entries(properties).map(([name, property], idx) => (
                <Property
                  key={idx}
                  name={name}
                  renderDepth={renderDepth}
                  required={requiredMap[name]}
                  propertiesPrimary={schemaDiffPrimary}
                  mayBePropertyDiffResult={property}
                  entityPath={`${entityPath}/properties/${name}`}
                />
              ))}

            {!showExample && schemaDiff.additionalProperties && (
              <SchemaAdditionalProperties
                renderDepth={renderDepth}
                parentPrimary={schemaDiffPrimary}
                entityPath={`${entityPath}/additionalProperties`}
                eitherAdditionalPropertiesDiffResult={
                  schemaDiff.additionalProperties
                }
              />
            )}

            {!showExample &&
              [
                { name: "not", value: schemaDiff.not },
                { name: "oneOf", value: schemaDiff.oneOf },
                { name: "allOf", value: schemaDiff.allOf },
                { name: "anyOf", value: schemaDiff.anyOf },
              ].map(
                ({ name, value }) =>
                  value && (
                    <VecProperty
                      key={name}
                      name={name}
                      renderDepth={renderDepth}
                      mayBeSchemaVecDiffResult={value}
                      parentPrimary={schemaDiffPrimary}
                      entityPath={`${entityPath}/${name}`}
                    />
                  )
              )}
          </Box>
        )}
      </Box>
    </Box>
  );
};

export default Schema;
