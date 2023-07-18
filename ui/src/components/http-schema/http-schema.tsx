import React, { useEffect, useState } from "react";

import { useDebounce } from "usehooks-ts";

import { Flex, Box, HStack, Input, Collapse } from "@chakra-ui/react";
import {
  HttpSchemaDiff,
  SchemaDiff,
  ParameterDiff,
  OperationDiff,
  PathDiff,
  ResponseDiff,
  RequestBodyDiff,
} from "./models";

import Operation from "./operation";

import OperationHeader from "./operation/operation-header";

import {
  DiffResult,
  DiffResultIs,
  MayBeRefDiff,
  MayBeRefDiffIs,
  PrimaryOperation,
  valueOfApplied,
  valueOf,
} from "./common";
import HttpSchemaFilters from "./http-schema-filters";

const makeSchemaDeref: (
  diff: HttpSchemaDiff
) => (ref: string) => [string, DiffResult<SchemaDiff>] | undefined = (
  diff: HttpSchemaDiff
) => {
  const deref = (ref: string): [string, DiffResult<SchemaDiff>] | undefined => {
    if (ref.startsWith("#/components/schemas/")) {
      const schemaName = ref.replace("#/components/schemas/", "");

      const components = valueOf(diff.components!);
      if (!components) {
        return;
      }

      const schemas = valueOf(components.schemas!);
      if (!schemas) {
        return;
      }

      const mayBeSchemaDiffResult = schemas[schemaName];
      if (!mayBeSchemaDiffResult) {
        return;
      }

      const mayBeSchema = valueOf(mayBeSchemaDiffResult);
      if (!mayBeSchema) {
        return;
      }

      if (mayBeSchema.t === MayBeRefDiffIs.Value) {
        return [schemaName, mayBeSchema.v];
      }

      return;
    }
  };

  return deref;
};

const makeParameterDeref: (
  diff: HttpSchemaDiff
) => (ref: string) => [string, DiffResult<ParameterDiff>] | undefined = (
  diff: HttpSchemaDiff
) => {
  const deref = (
    ref: string
  ): [string, DiffResult<ParameterDiff>] | undefined => {
    if (ref.startsWith("#/components/parameters/")) {
      const parameterName = ref.replace("#/components/parameters/", "");

      const components = valueOf(diff.components!);
      if (!components) {
        return;
      }

      const parameters = valueOf(components.parameters!);
      if (!parameters) {
        return;
      }

      const mayBeParameter = valueOf(parameters[parameterName]);
      if (!mayBeParameter) {
        return;
      }

      if (mayBeParameter.t === MayBeRefDiffIs.Value) {
        return [parameterName, mayBeParameter.v];
      }

      return;
    }
  };

  return deref;
};

const makeResponseDeref: (
  diff: HttpSchemaDiff
) => (ref: string) => [string, DiffResult<ResponseDiff>] | undefined = (
  diff: HttpSchemaDiff
) => {
  const deref = (
    ref: string
  ): [string, DiffResult<ResponseDiff>] | undefined => {
    if (ref.startsWith("#/components/responses/")) {
      const key = ref.replace("#/components/responses/", "");

      const components = valueOf(diff.components!);
      if (!components) {
        return;
      }

      const responses = valueOf(components.responses!);
      if (!responses) {
        return;
      }

      const mayBeResponseDiffResult = responses[key];
      if (!mayBeResponseDiffResult) {
        return;
      }

      const mayBeResponse = valueOf(mayBeResponseDiffResult);
      if (!mayBeResponse) {
        return;
      }

      if (mayBeResponse.t === MayBeRefDiffIs.Value) {
        return [key, mayBeResponse.v];
      }

      return;
    }
  };

  return deref;
};

const makeRequestBodyDeref: (
  diff: HttpSchemaDiff
) => (ref: string) => [string, DiffResult<RequestBodyDiff>] | undefined = (
  diff: HttpSchemaDiff
) => {
  const deref = (
    ref: string
  ): [string, DiffResult<RequestBodyDiff>] | undefined => {
    if (ref.startsWith("#/components/requestBodies/")) {
      const key = ref.replace("#/components/requestBodies/", "");

      const components = valueOf(diff.components!);
      if (!components) {
        return;
      }

      const requestBodies = valueOf(components.requestBodies!);
      if (!requestBodies) {
        return;
      }

      const mayBeRequestBodiesDiffResult = requestBodies[key];
      if (!mayBeRequestBodiesDiffResult) {
        return;
      }

      const mayBeRequestBody = valueOf(mayBeRequestBodiesDiffResult);
      if (!mayBeRequestBody) {
        return;
      }

      if (mayBeRequestBody.t === MayBeRefDiffIs.Value) {
        return [key, mayBeRequestBody.v];
      }

      return;
    }
  };

  return deref;
};

export const DerefSchemaContext = React.createContext<
  ((ref: string) => [string, DiffResult<SchemaDiff>] | undefined) | null
>(null);
export const DerefParameterContext = React.createContext<
  ((ref: string) => [string, DiffResult<ParameterDiff>] | undefined) | null
>(null);
export const DerefResponseContext = React.createContext<
  ((ref: string) => [string, DiffResult<ResponseDiff>] | undefined) | null
>(null);
export const DerefRequestBodyContext = React.createContext<
  ((ref: string) => [string, DiffResult<RequestBodyDiff>] | undefined) | null
>(null);

export const FocusPathContext = React.createContext<string | null>(null);

interface HttpSchemaProps {
  diff: HttpSchemaDiff;

  focusPath?: string;

  showSearch?: boolean;
  showFilters?: boolean;
  defaultDiffTypes?: DiffResultIs[];
}

const HttpSchema: React.FC<HttpSchemaProps> = ({
  diff,

  focusPath = "",

  showSearch = true,
  showFilters = true,
  defaultDiffTypes = [],
}: HttpSchemaProps) => {
  useEffect(() => {
    if (focusPath) {
      setTimeout(() => {
        const element = document.getElementById(focusPath);
        if (element) {
          element.classList.add("pathFocused");

          element.scrollIntoView({ behavior: "smooth", block: "center" });

          setTimeout(() => {
            element.classList.remove("pathFocused");
          }, 5000);
        }
      }, 50);
    }
  }, [focusPath]);

  const [filterDiffTypes, setFilterDiffTypes] = useState<DiffResultIs[]>(
    focusPath ? [] : defaultDiffTypes
  );

  const [filtered, setFiltered] = useState<boolean>(!showSearch);

  const setFilters = (filters: DiffResultIs[]) => {
    setFilterDiffTypes(filters);
    setFiltered(true);
  };

  const [search, setSearch] = useState<string | null>(null);

  const searchDebounced = useDebounce<string | null>(search, 350);

  if (!diff.paths) {
    return <Box>No paths found</Box>;
  }

  let [paths, primary] = valueOfApplied(diff.paths, PrimaryOperation.Same);
  if (!paths) {
    return <Box>No paths found</Box>;
  }

  let apiPaths = Object.entries(paths).filter(([key, _]) =>
    searchDebounced && searchDebounced.length > 1
      ? key.toLowerCase().includes(searchDebounced.toLowerCase())
      : true
  );

  const derefSchema = makeSchemaDeref(diff);
  const derefResponse = makeResponseDeref(diff);
  const derefParameter = makeParameterDeref(diff);
  const derefRequestBody = makeRequestBodyDeref(diff);

  let renderPaths = apiPaths
    .map(([key, mayBePathDiffResult]) => {
      const [mayBePath, mayBePathPrimary] = valueOfApplied(
        mayBePathDiffResult,
        primary
      ) as [MayBeRefDiff<PathDiff> | null, PrimaryOperation];

      if (!mayBePath) {
        return null;
      }

      const pathDiffResult =
        mayBePath.t === MayBeRefDiffIs.Value ? mayBePath.v : null;
      if (!pathDiffResult) {
        return null;
      }

      const [path, pathPrimary] = valueOfApplied(
        pathDiffResult,
        mayBePathPrimary
      );

      if (!path) {
        return null;
      }

      return [
        { operationDiffResult: path.get, methodName: "get" },
        { operationDiffResult: path.post, methodName: "post" },
        { operationDiffResult: path.put, methodName: "put" },
        { operationDiffResult: path.patch, methodName: "patch" },
        { operationDiffResult: path.delete, methodName: "delete" },
        { operationDiffResult: path.head, methodName: "head" },
        { operationDiffResult: path.options, methodName: "options" },
        { operationDiffResult: path.trace, methodName: "trace" },
      ].map(({ operationDiffResult, methodName }) => {
        if (!operationDiffResult) {
          return null;
        }

        if (
          filterDiffTypes.length > 0 &&
          !filterDiffTypes.includes(operationDiffResult.t)
        ) {
          return null;
        }

        return (
          <ApiPath
            path={key}
            focusPath={focusPath}
            key={key + methodName}
            methodName={methodName}
            parentPrimary={pathPrimary}
            operationDiffResult={operationDiffResult}
          />
        );
      });
    })
    .flat()
    .filter((path) => path !== null);

  return (
    <FocusPathContext.Provider value={focusPath}>
      <DerefRequestBodyContext.Provider value={derefRequestBody}>
        <DerefSchemaContext.Provider value={derefSchema}>
          <DerefResponseContext.Provider value={derefResponse}>
            <DerefParameterContext.Provider value={derefParameter}>
              <Flex width="100%" flexDirection="column" alignItems="flex-start">
                <HStack
                  width="100%"
                  display="flex"
                  maxWidth={1120}
                  alignItems="baseline"
                >
                  {showSearch && (
                    <Input
                      mb={3}
                      borderColor="gray.300"
                      placeholder="Type here to search ..."
                      onChange={(e: any) => setSearch(e.target.value)}
                    />
                  )}
                  {showFilters && (
                    <HttpSchemaFilters
                      defaults={filterDiffTypes}
                      onFiltersChanged={setFilters}
                    />
                  )}
                </HStack>

                {filtered && renderPaths}
              </Flex>
            </DerefParameterContext.Provider>
          </DerefResponseContext.Provider>
        </DerefSchemaContext.Provider>
      </DerefRequestBodyContext.Provider>
    </FocusPathContext.Provider>
  );
};

export default HttpSchema;

interface ApiPathProps {
  path: string;
  methodName: string;

  focusPath: string;

  parentPrimary: PrimaryOperation;

  operationDiffResult: DiffResult<OperationDiff>;
}

const ApiPath: React.FC<ApiPathProps> = ({
  path,
  focusPath,
  parentPrimary,

  methodName,
  operationDiffResult,
}: ApiPathProps) => {
  const entityPath = `#paths/${path}/${methodName}`;

  const [isExpanded, setIsExpanded] = useState<boolean>(
    focusPath.startsWith(entityPath)
  );

  const [operation, operationPrimary] = valueOfApplied(
    operationDiffResult,
    parentPrimary
  );
  if (!operation) {
    return null;
  }

  const bg =
    operationDiffResult.t === DiffResultIs.Added
      ? "green.400"
      : operationDiffResult.t === DiffResultIs.Removed
      ? "red.400"
      : operationDiffResult.t === DiffResultIs.Updated
      ? "orange.400"
      : "#5E6B7F";
  const bc =
    operationDiffResult.t === DiffResultIs.Added
      ? "green.700"
      : operationDiffResult.t === DiffResultIs.Removed
      ? "red.700"
      : operationDiffResult.t === DiffResultIs.Updated
      ? "orange.700"
      : "gray.600";

  const bga =
    operationDiffResult.t === DiffResultIs.Added
      ? "green.50"
      : operationDiffResult.t === DiffResultIs.Removed
      ? "red.50"
      : operationDiffResult.t === DiffResultIs.Updated
      ? "gray.100"
      : "gray.100";

  return (
    <Box display="flex" alignItems="stretch" width="100%" mb={2}>
      <Box
        p={0}
        bg={bg}
        mb={"auto"}
        width="100%"
        maxWidth={1120}
        id={entityPath}
        border="1px solid"
        borderRadius={5}
        borderColor={bc}
        overflow="hidden"
        _hover={{ bg }}
      >
        <OperationHeader
          path={path}
          methodName={methodName}
          operation={operation}
          operationPrimary={operationPrimary}
          onClick={() => setIsExpanded(!isExpanded)}
        />

        <Collapse in={isExpanded} animateOpacity={false} unmountOnExit>
          {isExpanded && (
            <Box backgroundColor={bga}>
              <Operation
                entityPath={entityPath}
                parentPrimary={operationPrimary}
                operationDiffResult={operationDiffResult}
                colorize={operationDiffResult.t === DiffResultIs.Updated}
              />
            </Box>
          )}
        </Collapse>
      </Box>
    </Box>
  );
};
