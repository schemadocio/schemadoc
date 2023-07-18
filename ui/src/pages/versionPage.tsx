import React, { useContext } from "react";

import { useParams } from "react-router-dom";
import VersionView from "../components/versions/versionView";
import { DiffResultIs } from "../components/http-schema/common";
import ProjectContext from "../components/projects/projectContext";

interface VersionPageProps {}

const VersionPage: React.FC<VersionPageProps> = () => {
  const project = useContext(ProjectContext);

  const { versionId } = useParams();

  if (!versionId || !project) {
    return null;
  }

  return (
    <VersionView
      project={project}
      versionId={+versionId}
      options={{
        defaultDiffTypes: [
          DiffResultIs.Added,
          DiffResultIs.Updated,
          DiffResultIs.Removed,
        ],
      }}
    />
  );
};

export default VersionPage;
