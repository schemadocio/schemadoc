import React from "react";

import { Box, Text } from "@chakra-ui/react";

import {
  valueOf,
  valueOfApplied,
  DiffResult,
  getBgColor,
  PrimaryOperation,
  DiffResultIs,
} from "../common";

interface SchemaAttributeProps {
  name: string;
  renderName?: boolean;
  parentPrimary: PrimaryOperation;
  attributeDiffResult: DiffResult<any>;
}

export const SchemaAttribute: React.FC<SchemaAttributeProps> = ({
  name,
  renderName = true,
  parentPrimary,
  attributeDiffResult,
}: SchemaAttributeProps) => {
  const [attribute, attributePrimary] = valueOfApplied(
    attributeDiffResult,
    parentPrimary
  );

  let oldAttribute;
  if (attributeDiffResult.t === DiffResultIs.Updated) {
    const old = attributeDiffResult.v[1] || null;
    oldAttribute = old && valueOf(old);
  }

  const backgroundColor = getBgColor(attributeDiffResult, parentPrimary);

  return (
    <Box
      px={0.5}
      py="1px"
      mb="1px"
      display="flex"
      borderRadius={3}
      color={"#5E6B7F"}
      width="fit-content"
      fontStyle="italic"
      alignItems="baseline"
      backgroundColor={backgroundColor}
    >
      {renderName && (
        <>
          <Text as="span" fontWeight="medium">{`${name}`}</Text>
          <Text as="span" mr={1}>
            :
          </Text>
        </>
      )}
      {oldAttribute && (
        <>
          <Text as="span" bgColor="red.100" borderRadius={3}>
            <SchemaAttributeValue
              attribute={oldAttribute}
              attributePrimary={attributePrimary}
            />
          </Text>
          <Text as="span" mr={1} ml={1}>
            →
          </Text>
        </>
      )}
      {oldAttribute ? (
        <Text as="span" bgColor="green.100" borderRadius={3}>
          <SchemaAttributeValue
            attribute={attribute}
            attributePrimary={attributePrimary}
          />
        </Text>
      ) : (
        <SchemaAttributeValue
          attribute={attribute}
          attributePrimary={attributePrimary}
        />
      )}
    </Box>
  );
};

export default SchemaAttribute;

interface SchemaAttributeValueProps {
  attribute: any;
  attributePrimary: PrimaryOperation;
}

const SchemaAttributeValue: React.FC<SchemaAttributeValueProps> = ({
  attribute,
  attributePrimary,
}) => {
  let value = null;
  if (Array.isArray(attribute)) {
    value = (
      <Box>
        <Text key={"start"} as="span">
          {"["}
        </Text>

        {attribute.map((item, idx) => {
          let value = item;
          let backgroundColor = undefined;

          if (typeof item === "object") {
            let val = valueOf(item);
            value = JSON.stringify(val === null ? item : val, null, 2);
            backgroundColor = getBgColor(item, attributePrimary);
          }

          if (typeof value === "boolean") {
            value = value ? "true" : "false";
          }

          if (value === "") {
            value = '""';
          }

          return (
            <Text
              key={idx}
              borderRadius={3}
              width="fit-content"
              backgroundColor={backgroundColor}
              ml={attribute.length > 1 ? 2 : 0}
              p={attribute.length > 1 ? 0.5 : 0}
              mb={attribute.length > 1 ? "1px" : 0}
              display={attribute.length > 1 ? "block" : "inline-block"}
            >
              {value}
              {idx === attribute.length - 1 ? null : ","}
            </Text>
          );
        })}

        <Text key={"end"} as="span">
          {"]"}
        </Text>
      </Box>
    );
  } else if (typeof attribute === "boolean") {
    value = <Text as="span">{attribute ? "true" : "false"}</Text>;
  } else if (typeof attribute === "object") {
    value = (
      <Text as="span" whiteSpace="pre-wrap">
        {JSON.stringify(attribute, null, 2)}
      </Text>
    );
  } else if (typeof attribute === "string") {
    if (attribute === "") {
      attribute = '""';
    }
    value = (
      <Text as="span" whiteSpace="pre-wrap" wordBreak="break-word">
        {attribute}
      </Text>
    );
  } else {
    value = (
      <Text as="span" whiteSpace="pre-wrap">
        {String(attribute)}
      </Text>
    );
  }

  return value;
};
