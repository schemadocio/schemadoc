import React, { useEffect, useState } from "react";
import api from "../../api";

import Loading from "../loading";
import VersionMeta from "./versionMeta";
import { HttpSchemaDiff } from "../http-schema/models";
import { DiffResultIs } from "../http-schema/common";
import { VersionStatistics } from "./models";
import { Box } from "@chakra-ui/react";

interface VersionCompareProps {
  sourceId: number;
  targetId: number;
  projectSlug: string;
  sourceBranch: string;
  targetBranch: string;

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
  sourceBranch,
  targetBranch,

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
      .compare(projectSlug, sourceBranch, sourceId, targetBranch, targetId)
      .then(({ data }) => setDiff(data));
  }, [projectSlug, sourceBranch, sourceId, targetBranch, targetId, allowSame]);

  if (sourceId === targetId && !allowSame) {
    return <Box>Cannot compare same versions</Box>;
  }

  if (!diff) {
    return <Loading text="custom diff" />;
  }

  return (
    <VersionMeta diff={diff.diff} statistics={diff.statistics} {...options} />
  );
};
