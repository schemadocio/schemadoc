import React from "react";

import { Box } from "@chakra-ui/react";

import { SchemaDiff } from "../models";
import SchemaAttribute from "./schema-attribute";
import { PrimaryOperation } from "../common";

const BASIC_ATTRS_FIELDS = [
  "description",
  "title",

  "default",
  "enum",
  "pattern",
  "readOnly",
  "writeOnly",
  "minLength",
  "maxLength",
  "multipleOf",
  "example",
];

interface SchemaAttributeProps {
  exclude?: string[];
  schemaDiff: SchemaDiff;
  parentPrimary: PrimaryOperation;
}

export const SchemaAttributes: React.FC<SchemaAttributeProps> = ({
  schemaDiff,
  exclude = [],
  parentPrimary,
}: SchemaAttributeProps) => {
  let fields = BASIC_ATTRS_FIELDS.filter((field) =>
    !!exclude ? !exclude.includes(field) : true
  );

  if (fields.length === 0) {
    return null;
  }

  return (
    <Box lineHeight="normal">
      {fields.map((field) => {
        const attributeDiffResult = schemaDiff[field];
        if (!attributeDiffResult) {
          return null;
        }
        return (
          <SchemaAttribute
            key={field}
            name={field}
            parentPrimary={parentPrimary}
            renderName={field !== "description"}
            attributeDiffResult={attributeDiffResult}
          />
        );
      })}
    </Box>
  );
};

export default SchemaAttributes;
