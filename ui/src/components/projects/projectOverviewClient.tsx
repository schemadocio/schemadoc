import React, { useCallback, useEffect } from "react";
import { Box, Select, Text } from "@chakra-ui/react";
import { useLocation, useSearchParams } from "react-router-dom";

import Loading from "../loading";
import { Dependency, Project } from "./models";
import { DiffResultIs } from "../http-schema/common";
import { VersionCompare } from "../versions/versionCompare";

import api from "../../api";

interface ProjectOverviewClientProps {
  project: Project;
}

const ProjectOverviewClient: React.FC<ProjectOverviewClientProps> = ({
  project,
}: ProjectOverviewClientProps) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const searchDep = searchParams.get("dep");
  const searchSourceId = searchParams.get("src");
  const searchTargetId = searchParams.get("tgt");

  let selectDependency = useCallback(
    (slug: string) => {
      setSearchParams((prev) => ({ ...prev, dep: slug }));
    },
    [setSearchParams]
  );

  useEffect(() => {
    if (searchDep) {
      return;
    }

    let deps = project.dependencies;
    if (deps && deps.length > 0) {
      selectDependency(
        deps.find((d) => d.breaking)?.project ||
          deps.find((d) => d.outdated)?.project ||
          deps[0].project
      );
    }
  }, [searchDep, project.dependencies, selectDependency]);

  let selected = project.dependencies?.find((d) => d.project === searchDep);

  if (!selected || !searchDep) {
    return <Box p={3}>No dependencies found for the project</Box>;
  }

  return (
    <Box pr={3} flex={1} display="flex" borderRadius={5} flexDirection="column">
      <Box
        p={3}
        height="58px"
        display="flex"
        bgColor="gray.50"
        maxWidth={1120}
        borderRadius={5}
        alignItems="center"
      >
        <Text mr={3}>Dependency:</Text>
        <Select
          maxWidth="440px"
          defaultValue={searchDep}
          onChange={(e) => selectDependency(e.target.value)}
        >
          {project.dependencies &&
            project.dependencies.map((dep) => (
              <option value={dep.project} key={dep.project}>
                {dep.project}
              </option>
            ))}
        </Select>
        <Text ml={3} whiteSpace="nowrap">
          <Text as="span">Compare</Text>{" "}
          <Text as="span" fontWeight="medium">
            [{searchSourceId}]
          </Text>{" "}
          <Text as="span">with</Text>{" "}
          <Text as="span" fontWeight="medium">
            [{searchTargetId}]
          </Text>
        </Text>
      </Box>

      <DependencyDiff dependency={selected} />
    </Box>
  );
};

export default ProjectOverviewClient;

interface DependencyDiffProps {
  dependency: Dependency;
}

const DependencyDiff: React.FC<DependencyDiffProps> = ({ dependency }) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const searchSourceId = searchParams.get("src");
  const searchTargetId = searchParams.get("tgt");

  const { hash } = useLocation();

  useEffect(() => {
    if (searchSourceId && searchTargetId) {
      return;
    }

    api.versions.list(dependency.project).then(({ data }) => {
      if (data.length > 0) {
        setSearchParams((prev) => ({
          ...prev,
          dep: dependency.project,
          src: String(dependency.version),
          tgt: String(data[0].id),
        }));
      } else {
        setSearchParams((prev) => ({
          ...prev,
          dep: dependency.project,
          src: String(dependency.version),
          tgt: String(dependency.version),
        }));
      }
    });
  }, [dependency, searchSourceId, searchTargetId, setSearchParams]);

  if (!searchSourceId || !searchTargetId) {
    return <Loading text="versions" />;
  }

  return (
    <VersionCompare
      projectSlug={dependency.project}
      sourceId={+searchSourceId}
      targetId={+searchTargetId}
      focusPath={decodeURI(hash)}
      defaultDiffTypes={[
        DiffResultIs.Added,
        DiffResultIs.Updated,
        DiffResultIs.Removed,
      ]}
      allowSame={true}
    />
  );
};
