import React, { useContext } from "react";
import { useParams } from "react-router-dom";

import Loading from "../components/loading";
import { ProjectContext } from "../components/projects/projectContext";
import { VersionCompare } from "../components/versions/versionCompare";

interface VersionsComparePageProps {}

const VersionsComparePage: React.FC<VersionsComparePageProps> = () => {
  const project = useContext(ProjectContext);

  const { sourceId, sourceBranch, targetId, targetBranch } = useParams();

  if (!project || !sourceId || !targetId || !sourceBranch || !targetBranch) {
    return <Loading text="parameters" />;
  }

  return (
    <VersionCompare
      sourceId={+sourceId}
      targetId={+targetId}
      sourceBranch={sourceBranch}
      targetBranch={targetBranch}
      projectSlug={project.slug}
    />
  );
};

export default VersionsComparePage;
