import { Box, Text, Select, Divider, Flex } from "@chakra-ui/react";
import React, { useContext, useState } from "react";
import { MediaTypeDiff, RequestBodyDiff } from "../models";

import {
  derefValue,
  DiffResult,
  getBgColor,
  MapDiff,
  MayBeRefDiff,
  PrimaryOperation,
  selectValue,
  valueOf,
  valueOfApplied,
} from "../common";
import Schema from "../schema";
import { DerefRequestBodyContext } from "../http-schema";

interface OperationBodyProps {
  entityPath: string;
  parentPrimary: PrimaryOperation;
  mayBeRequestBodyDiffResult: DiffResult<MayBeRefDiff<RequestBodyDiff>>;
}

export const OperationBody: React.FC<OperationBodyProps> = ({
  entityPath,
  parentPrimary,
  mayBeRequestBodyDiffResult,
}: OperationBodyProps) => {
  const derefRequestBody = useContext(DerefRequestBodyContext);
  if (!derefRequestBody) {
    return null;
  }

  const [mayBeRequestBody, mayBeRequestBodyPrimary] = valueOfApplied(
    mayBeRequestBodyDiffResult,
    parentPrimary
  );
  if (!mayBeRequestBody) {
    return null;
  }

  const requestBodyDiffResult = derefValue(mayBeRequestBody, derefRequestBody);
  if (!requestBodyDiffResult) {
    return null;
  }

  const [requestBody, requestBodyPrimary] = valueOfApplied(
    requestBodyDiffResult,
    mayBeRequestBodyPrimary
  );
  if (!requestBody) {
    return null;
  }

  const parentEntityPath = `${entityPath}/requestBody`;
  const contentEntityPath = `${parentEntityPath}/content`;

  let requiredStyle = selectValue(
    requestBody.required,
    requestBodyPrimary,
    (value) => ({}),
    (value) => ({ bgColor: "green.100" }),
    (value) => ({ bgColor: "rgba(255, 200, 51, 0.3)" }),
    (value) => ({ bgColor: "red.100" })
  );

  let required = requiredStyle && (
    <Text color="red" display="inline" as="span" {...requiredStyle}>
      *
    </Text>
  );

  const bgColor = getBgColor(mayBeRequestBodyDiffResult, parentPrimary);

  return (
    <Box id={parentEntityPath} bgColor={bgColor}>
      <Divider mb={4} />
      {requestBody.description && (
        <Text px={6} pt={3}>
          {valueOf(requestBody.description)}
        </Text>
      )}

      {requestBody.content && (
        <OperationBodyContent
          required={required}
          entityPath={contentEntityPath}
          parentPrimary={requestBodyPrimary}
          contentDiffResult={requestBody.content}
        />
      )}
      <Divider />
    </Box>
  );
};

export default OperationBody;

interface OperationBodyContentProps {
  required: React.ReactElement | null;
  entityPath: string;
  contentDiffResult: DiffResult<MapDiff<MediaTypeDiff>>;
  parentPrimary: PrimaryOperation;
}

export const OperationBodyContent: React.FC<OperationBodyContentProps> = ({
  required,
  entityPath,
  contentDiffResult,
  parentPrimary,
}: OperationBodyContentProps) => {
  let [mediaTypeMap, mediaTypeMapPrimary] = valueOfApplied(
    contentDiffResult,
    parentPrimary
  );

  const mediaTypes = Object.keys(mediaTypeMap || { "application/json": {} });

  const [mediaTypeName, setMediaTypeName] = useState<string>(mediaTypes[0]);

  if (!mediaTypeMap || !mediaTypeName) {
    return null;
  }

  let [mediaType, mediaTypePrimary] = valueOfApplied(
    mediaTypeMap[mediaTypeName],
    mediaTypeMapPrimary
  );

  if (!mediaType || !mediaTypePrimary) {
    return null;
  }

  const selectorBgColor = getBgColor(contentDiffResult, parentPrimary, true);

  const selectedBgColor = getBgColor(
    mediaTypeMap[mediaTypeName],
    mediaTypePrimary,
    true
  );

  const mediaTypeSelectOptions = mediaTypes.map((mediaType) => {
    if (!mediaTypeMap) {
      return null;
    }

    let modifier = selectValue(
      mediaTypeMap[mediaType],
      parentPrimary,
      (value) => "",
      (value) => "[+]",
      (value) => "[~]",
      (value) => "[-]"
    );

    return (
      <option value={mediaType} key={mediaType}>
        {modifier} {mediaType}
      </option>
    );
  });

  const mediaTypeId = `${entityPath}/${mediaTypeName}`;
  const mediaTypeSchemaId = `${mediaTypeId}/schema`;

  return (
    <Flex id={entityPath} p={6} flexDirection="column">
      <Flex p={1} bgColor={selectorBgColor}>
        <Text fontWeight="medium" color="gray.600" width={180}>
          Request body {required}
        </Text>

        {mediaType.schema && (
          <Box ml={1} flex={1}>
            <Schema
              open={true}
              renderDepth={5}
              fieldName={"Body"}
              entityPath={mediaTypeSchemaId}
              parentPrimary={mediaTypePrimary}
              mayBeSchemaDiffResult={mediaType.schema}
            />
          </Box>
        )}
      </Flex>

      <Select
        mr={1}
        mt={2}
        ml="auto"
        size="sm"
        color="gray.600"
        maxWidth="300px"
        id={mediaTypeId}
        bgColor={selectedBgColor}
        onChange={(e: any) => setMediaTypeName(e.target.value)}
      >
        {mediaTypeSelectOptions}
      </Select>
    </Flex>
  );
};
