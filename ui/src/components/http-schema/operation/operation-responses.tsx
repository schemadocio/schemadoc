import {
  Tr,
  Td,
  Text,
  Select,
  Table,
  TableCaption,
  Thead,
  Th,
  Tbody,
  Link,
  Box,
} from "@chakra-ui/react";
import React, { useContext, useState } from "react";

import { MediaTypeDiff, ResponseDiff } from "../models";

import Schema from "../schema";

import {
  DiffResult,
  MayBeRefDiff,
  getBgColor,
  MapDiff,
  valueOf,
  valueOfApplied,
  derefValue,
  PrimaryOperation,
  selectValue,
} from "../common";
import { DerefResponseContext } from "../http-schema";

interface OperationResponsesProps {
  entityPath: string;
  parentPrimary: PrimaryOperation;
  mayBeResponsesDiffResultRef: DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>;
}

export const OperationResponses: React.FC<OperationResponsesProps> = ({
  entityPath,
  parentPrimary,
  mayBeResponsesDiffResultRef,
}: OperationResponsesProps) => {
  const [responses, responsesPrimary] = valueOfApplied(
    mayBeResponsesDiffResultRef,
    parentPrimary
  );
  if (!responses || Object.entries(responses).length === 0) {
    return null;
  }

  const id = `${entityPath}/responses`;

  return (
    <Table id={id}>
      <TableCaption placement="top" fontSize={16} m={0} p={3}>
        Responses
      </TableCaption>
      <Thead>
        <Tr>
          <Th width={180}>Status code</Th>
          <Th>Schema</Th>
        </Tr>
      </Thead>
      <Tbody>
        {Object.entries(responses).map(([statusCode, responseDiffResult]) => (
          <OperationResponse
            entityPath={id}
            key={statusCode}
            statusCode={statusCode}
            parentPrimary={responsesPrimary}
            responseDiffResultRef={responseDiffResult}
          />
        ))}
      </Tbody>
    </Table>
  );
};

export default OperationResponses;

interface OperationResponseProps {
  entityPath: string;
  statusCode: string;
  parentPrimary: PrimaryOperation;
  responseDiffResultRef: DiffResult<MayBeRefDiff<ResponseDiff>>;
}

export const OperationResponse: React.FC<OperationResponseProps> = ({
  entityPath,
  statusCode,
  parentPrimary,
  responseDiffResultRef,
}: OperationResponseProps) => {
  const derefResponse = useContext(DerefResponseContext);
  if (!derefResponse) {
    return null;
  }

  const [mayBeResponseDiffResult, mayBeResponseDiffResultPrimary] =
    valueOfApplied(responseDiffResultRef, parentPrimary);
  if (!mayBeResponseDiffResult) {
    return null;
  }

  const responseDiffResult = derefValue(mayBeResponseDiffResult, derefResponse);
  if (!responseDiffResult) {
    return null;
  }

  let [response, responsePrimary] = valueOfApplied(
    responseDiffResult,
    mayBeResponseDiffResultPrimary
  );
  if (!response || !responsePrimary) {
    return null;
  }

  const mediaTypeId = `${entityPath}/${statusCode}`;

  const backgroundColor = getBgColor(responseDiffResult, responsePrimary);

  const description = response.description && valueOf(response.description);
  const descriptionBgColor =
    response.description && getBgColor(response.description, responsePrimary);

  return (
    <Tr id={mediaTypeId} key={mediaTypeId} backgroundColor={backgroundColor}>
      <Td verticalAlign="baseline">
        <Link href={mediaTypeId}>
          <Text fontWeight="medium" fontSize="md" py={2}>
            {statusCode}
          </Text>
        </Link>
      </Td>
      <Td pb={6}>
        {description && (
          <Text backgroundColor={descriptionBgColor} p={2}>
            {description}
          </Text>
        )}
        {response.content && (
          <OperationResponseSchema
            statusCode={statusCode}
            entityPath={mediaTypeId}
            parentPrimary={responsePrimary}
            contentDiffResult={response.content}
          />
        )}
      </Td>
    </Tr>
  );
};

interface OperationResponseSchemaProps {
  statusCode: string;
  entityPath: string;
  contentDiffResult: DiffResult<MapDiff<MediaTypeDiff>>;
  parentPrimary: PrimaryOperation;
}

const OperationResponseSchema: React.FC<OperationResponseSchemaProps> = ({
  entityPath,
  statusCode,
  contentDiffResult,
  parentPrimary,
}: OperationResponseSchemaProps) => {
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
      mediaTypePrimary,
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

  const mediaTypeContentId = `${entityPath}/content`;

  return (
    <Box bgColor={selectorBgColor} borderRadius={3} p={1}>
      {mediaType.schema && (
        <Schema
          open={true}
          renderDepth={5}
          fieldName={statusCode}
          entityPath={mediaTypeContentId}
          parentPrimary={mediaTypePrimary}
          mayBeSchemaDiffResult={mediaType.schema}
        />
      )}
      <Select
        onChange={(e: any) => setMediaTypeName(e.target.value)}
        bgColor={selectedBgColor}
        maxWidth="300px"
        color="gray.600"
        size="sm"
        ml="auto"
        mt={1}
      >
        {mediaTypeSelectOptions}
      </Select>
    </Box>
  );
};
