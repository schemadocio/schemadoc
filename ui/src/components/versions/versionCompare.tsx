import React, { useEffect, useState } from "react";
import api from "../../api";

import Loading from "../loading";
import VersionMeta from "./versionMeta";
import { HttpSchemaDiff } from "../http-schema/models";
import { DiffResultIs } from "../http-schema/common";
import { VersionStatistics } from "./models";

interface VersionCompareProps {
  sourceId: number;
  targetId: number;
  projectSlug: string;
  allowSame?: boolean;

  focusPath?: string;
  showSearch?: boolean;
  showFilters?: boolean;
  defaultDiffTypes?: DiffResultIs[];
}

export const VersionCompare: React.FC<VersionCompareProps> = ({
  sourceId,
  targetId,
  projectSlug,
  allowSame = false,
  ...options
}) => {
  const [diff, setDiff] = useState<{
    diff: HttpSchemaDiff;
    statistics: VersionStatistics;
  } | null>(null);

  useEffect(() => {
    if (sourceId === targetId && !allowSame) {
      return;
    }

    api.versions
      .compare(projectSlug, sourceId, targetId)
      .then(({ data }) => setDiff(data));
  }, [projectSlug, sourceId, targetId, allowSame]);

  if (!diff) {
    return <Loading text="custom diff" />;
  }

  return (
    <VersionMeta diff={diff.diff} statistics={diff.statistics} {...options} />
  );
};
