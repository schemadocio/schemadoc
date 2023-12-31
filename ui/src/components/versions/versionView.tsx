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
  branchName: string;
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
  branchName,
  versionId,
  options,
}) => {
  const [version, setVersion] = useState<Version | null>(null);
  const [diff, setDiff] = useState<HttpSchemaDiff | null>(null);

  useEffect(() => {
    api.versions
      .get(project.slug, branchName, versionId)
      .then(({ data }) => data.result && setVersion(data.result));
  }, [project.slug, branchName, versionId]);

  useEffect(() => {
    api.versions
      .getDiff(project.slug, branchName, versionId)
      .then(({ data }) => {
        setDiff(data);
      });
  }, [project.slug, branchName, versionId]);

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
