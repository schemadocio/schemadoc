import React, { useEffect, useState } from "react";
import api from "../../api";

import Loading from "../loading";
import VersionMeta from "./versionMeta";
import { HttpSchemaDiff } from "../http-schema/models";
import { DiffResultIs } from "../http-schema/common";
import { Project } from "../projects/models";

interface VersionCompareProps {
  project: Project;
  sourceId: number;
  targetId: number;
  allowSame?: boolean;

  focusPath?: string;
  showSearch?: boolean;
  showFilters?: boolean;
  defaultDiffTypes?: DiffResultIs[];
}

export const VersionCompare: React.FC<VersionCompareProps> = ({
  project,
  sourceId,
  targetId,
  allowSame = false,
  ...options
}) => {
  const [diff, setDiff] = useState<HttpSchemaDiff | null>(null);

  useEffect(() => {
    if (sourceId === targetId && !allowSame) {
      return;
    }

    api.versions
      .compare(project.slug, sourceId, targetId)
      .then(({ data }) => setDiff(data));
  }, [project, sourceId, targetId, allowSame]);

  if (!diff) {
    return <Loading text="custom diff" />;
  }

  return <VersionMeta diff={diff} {...options} />;
};
