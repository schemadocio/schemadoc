import React from "react";
import { useLocation } from "react-router-dom";

import HttpSchema from "../http-schema/http-schema";
import { DiffResultIs } from "../http-schema/common";
import { HttpSchemaDiff } from "../http-schema/models";
import { VersionStatistics } from "./models";

interface VersionMetaProps {
  diff: HttpSchemaDiff;
  statistics: VersionStatistics;
  focusPath?: string;
  showSearch?: boolean;
  showFilters?: boolean;
  defaultDiffTypes?: DiffResultIs[];
}

const VersionMeta: React.FC<VersionMetaProps> = React.memo(
  ({
    diff,
    statistics,
    focusPath,
    showSearch,
    showFilters,
    defaultDiffTypes,
  }) => {
    const { hash } = useLocation();

    let actualFocusPath = focusPath || decodeURI(hash);

    return (
      <HttpSchema
        diff={diff}
        statistics={statistics}
        focusPath={actualFocusPath}
        showSearch={showSearch}
        showFilters={showFilters}
        defaultDiffTypes={defaultDiffTypes}
      />
    );
  }
);

export default VersionMeta;
