import React, { useContext } from "react";
import { useParams } from "react-router-dom";

import Loading from "../components/loading";
import { ProjectContext } from "../components/projects/projectContext";
import { VersionCompare } from "../components/versions/versionCompare";

interface CompareVersionsPageProps {}

const CompareVersionsPage: React.FC<CompareVersionsPageProps> = ({}) => {
  const project = useContext(ProjectContext);

  const { sourceId, targetId } = useParams();

  if (!project || !sourceId || !targetId) {
    return <Loading text="parameters" />;
  }

  return (
    <VersionCompare
      project={project}
      sourceId={+sourceId}
      targetId={+targetId}
    />
  );
};

export default CompareVersionsPage;
