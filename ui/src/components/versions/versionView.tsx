import React, { useEffect, useState } from "react";

import api from "../../api";

import { Box } from "@chakra-ui/react";

import Loading from "../loading";
import VersionMeta from "./versionMeta";
import { Project } from "../projects/models";
import { DiffResultIs } from "../http-schema/common";
import { HttpSchemaDiff } from "../http-schema/models";
import { Version } from "./models";

interface VersionViewProps {
  project: Project;
  versionId: number;
  options?: VersionViewOptions;
}

interface VersionViewOptions {
  focusPath?: string;

  showSearch?: boolean;
  showFilters?: boolean;
  defaultDiffTypes?: DiffResultIs[];
}

export const VersionView: React.FC<VersionViewProps> = ({
  project,
  versionId,
  options,
}) => {
  const [version, setVersion] = useState<Version | null>(null);
  const [diff, setDiff] = useState<HttpSchemaDiff | null>(null);

  useEffect(() => {
    api.versions
      .get(project.slug, versionId)
      .then(({ data }) => data && setVersion(data));
  }, [project.slug, versionId]);

  useEffect(() => {
    api.versions.getDiff(project.slug, versionId).then(({ data }) => {
      setDiff(data);
    });
  }, [versionId, project.slug]);

  if (!diff || !version) {
    return <Loading text="diff" />;
  }

  return (
    <Box width="100%" maxWidth={1120}>
      <VersionMeta
        diff={diff}
        statistics={version.statistics}
        focusPath={options?.focusPath}
        showSearch={options?.showSearch}
        showFilters={options?.showFilters}
        defaultDiffTypes={options?.defaultDiffTypes}
      />
    </Box>
  );
};

export default VersionView;
