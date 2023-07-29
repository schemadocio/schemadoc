import {
  Text,
  Table,
  TableCaption,
  Thead,
  Tr,
  Td,
  Th,
  Tbody,
  Link,
  Box,
} from "@chakra-ui/react";
import React, { useContext } from "react";
import { ParameterDiff } from "../models";

import { DerefParameterContext, DerefSchemaContext } from "../http-schema";

import {
  derefValue,
  DiffResult,
  getBgColor,
  MayBeRefDiff,
  PrimaryOperation,
  selectValue,
  valueOf,
  valueOfApplied,
  VecDiff,
} from "../common";
import { SchemaType } from "../schema/schema-type";
import { SchemaAttributes } from "../schema";
import RequiredBadge from "../components/required-badge";

interface OperationParametersProps {
  entityPath: string;
  parentPrimary: PrimaryOperation;
  parametersDiffResult: DiffResult<VecDiff<MayBeRefDiff<ParameterDiff>>>;
}

export const OperationParameters: React.FC<OperationParametersProps> = ({
  entityPath,
  parentPrimary,
  parametersDiffResult,
}: OperationParametersProps) => {
  const schemaDeref = useContext(DerefSchemaContext);
  const parameterDeref = useContext(DerefParameterContext);

  if (!parameterDeref || !schemaDeref) {
    return null;
  }

  const [parameters, parametersPrimary] = valueOfApplied(
    parametersDiffResult,
    parentPrimary
  );
  if (!parameters || parameters.length === 0) {
    return null;
  }

  const parentEntityPath = `${entityPath}/parameters`;

  return (
    <Table>
      <TableCaption placement="top" fontSize={16} m={0} p={3}>
        Request parameters
      </TableCaption>
      <Thead>
        <Tr>
          <Th width={180}>
            <Text ml={0.5}>Name</Text>
          </Th>
          <Th>
            <Text ml={1}>Description</Text>
          </Th>
        </Tr>
      </Thead>
      <Tbody>
        {parameters &&
          parameters.map(
            (parameter, idx) =>
              parameter && (
                <OperationParameter
                  key={idx}
                  idx={idx}
                  entityPath={parentEntityPath}
                  parentPrimary={parametersPrimary}
                  mayBeParameterDiffResult={parameter}
                />
              )
          )}
      </Tbody>
    </Table>
  );
};

export default OperationParameters;

interface OperationParameterProps {
  idx: number;
  entityPath: string;
  parentPrimary: PrimaryOperation;
  mayBeParameterDiffResult: DiffResult<MayBeRefDiff<ParameterDiff>>;
}

export const OperationParameter: React.FC<OperationParameterProps> = ({
  idx,
  entityPath,
  parentPrimary,
  mayBeParameterDiffResult,
}: OperationParameterProps) => {
  const schemaDeref = useContext(DerefSchemaContext);
  const parameterDeref = useContext(DerefParameterContext);

  if (!parameterDeref || !schemaDeref) {
    return null;
  }

  const [maybeParameter, maybeParameterPrimary] = valueOfApplied(
    mayBeParameterDiffResult,
    parentPrimary
  );
  if (!maybeParameter) {
    return null;
  }

  const parameterDiffResult = derefValue(maybeParameter, parameterDeref);
  if (!parameterDiffResult) {
    return null;
  }

  const [parameter, parameterPrimary] = valueOfApplied(
    parameterDiffResult,
    maybeParameterPrimary
  );
  if (!parameter) {
    return null;
  }

  const nameFontWeight =
    selectValue(
      parameter.required,
      parameterPrimary,
      (value) => (value ? "medium" : "normal"),
      (value) => (value ? "medium" : "normal"),
      (value) => (value ? "medium" : "normal"),
      (value) => "normal"
    ) || undefined;

  let schema = null;
  let schemaPrimary = null;
  if (parameter.schema) {
    const [mayBeRefSchema, mayBeRefSchemaPrimary] = valueOfApplied(
      parameter.schema,
      parameterPrimary
    );
    if (mayBeRefSchema) {
      const schemaEntity = derefValue(mayBeRefSchema, schemaDeref);
      if (schemaEntity) {
        [schema, schemaPrimary] = valueOfApplied(
          schemaEntity,
          mayBeRefSchemaPrimary
        );
      }
    }
  }

  const id = `${entityPath}/${idx}`;

  const backgroundColor = getBgColor(parameterDiffResult, parentPrimary);

  return (
    <Tr mb={2} id={id} key={id} backgroundColor={backgroundColor}>
      <Td verticalAlign="baseline" m={1} pr={2}>
        <Link href={id}>
          <Text fontWeight={nameFontWeight} fontSize="md" ml={0.5}>
            <Text as="span">{parameter.name}</Text>
            <RequiredBadge
              primary={parameterPrimary}
              required={parameter.required}
            />
          </Text>
        </Link>
        <Box fontWeight="bold" fontSize="xs" lineHeight="normal" py={1}>
          {schema && schemaPrimary && schema.type && (
            <SchemaType
              topmost={false}
              schema={schema}
              schemaPrimary={schemaPrimary}
            />
          )}
        </Box>
        <Text fontSize="xs" fontStyle="italic" color="gray.600" ml={0.5} lineHeight="normal">
          ({parameter.in})
        </Text>
      </Td>
      <Td verticalAlign="initial">
        <Text ml={1} whiteSpace="pre-wrap">
          {parameter.description && valueOf(parameter.description)}
        </Text>
        <Box mt={1} ml={1} fontSize={12}>
          {schema && schemaPrimary && (
            <SchemaAttributes
              schemaDiff={schema}
              exclude={["description"]}
              parentPrimary={schemaPrimary}
            />
          )}
        </Box>
      </Td>
    </Tr>
  );
};
