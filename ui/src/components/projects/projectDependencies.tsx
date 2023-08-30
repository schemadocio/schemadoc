import React, { useCallback, useEffect, useState } from "react";
import { Box, Select, Text, Link as ChakraLink } from "@chakra-ui/react";
import { Link, useLocation, useSearchParams } from "react-router-dom";

import Loading from "../loading";
import { Dependency, Project } from "./models";
import { DiffResultIs } from "../http-schema/common";
import { VersionCompare } from "../versions/versionCompare";

import api from "../../api";
import { BsArrowUpRight } from "react-icons/bs";

interface ProjectDependenciesProps {
  project: Project;
}

const ProjectDependencies: React.FC<ProjectDependenciesProps> = ({
  project,
}: ProjectDependenciesProps) => {
  const [depHasVersion, setDepHasVersion] = useState<boolean>(false);

  const [searchParams, setSearchParams] = useSearchParams();

  const searchDep = searchParams.get("dep");
  const searchSourceId = searchParams.get("src");
  const searchSourceBranch = searchParams.get("srcBranch");
  const searchTargetId = searchParams.get("tgt");
  const searchTargetBranch = searchParams.get("tgtBranch");

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

  useEffect(() => {
    if (!selected) {
      return;
    }

    api.versions
      .get(selected.project, selected.branch, selected.version)
      .then(() => setDepHasVersion(true))
      .catch(() => setDepHasVersion(false));
  }, [selected, setDepHasVersion]);

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

        <ChakraLink
          mx={2}
          as={Link}
          title="Go to the project page"
          to={`/projects/${selected.project}`}
        >
          <BsArrowUpRight size={20} />
        </ChakraLink>

        {depHasVersion && (
          <Text ml={1} whiteSpace="nowrap">
            <Text as="span">Compare</Text>{" "}
            <Text as="span" fontWeight="medium">
              {searchSourceBranch}: {searchSourceId}
            </Text>{" "}
            <Text as="span">with</Text>{" "}
            <Text as="span" fontWeight="medium">
              {searchTargetBranch}: {searchTargetId}
            </Text>
          </Text>
        )}
      </Box>

      {depHasVersion && <DependencyDiff dependency={selected} />}
      {!depHasVersion && (
        <Text my={3}>
          Wront version specified: '{selected.version}' could not be loaded
        </Text>
      )}
    </Box>
  );
};

export default ProjectDependencies;

interface DependencyDiffProps {
  dependency: Dependency;
}

const DependencyDiff: React.FC<DependencyDiffProps> = ({ dependency }) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const searchSourceId = searchParams.get("src");
  const searchSourceBranch = searchParams.get("srcBranch");
  const searchTargetId = searchParams.get("tgt");
  const searchTargetBranch = searchParams.get("tgtBranch");

  const { hash } = useLocation();

  useEffect(() => {
    if (
      searchSourceBranch &&
      searchSourceId &&
      searchTargetBranch &&
      searchTargetId
    ) {
      return;
    }

    api.versions
      .list(dependency.project, dependency.branch)
      .then(({ data }) => {
        if (data.result.length > 0) {
          setSearchParams((prev) => ({
            ...prev,
            dep: dependency.project,
            srcBranch: dependency.branch,
            src: String(dependency.version),
            tgtBranch: dependency.branch,
            tgt: String(data.result[0].id),
          }));
        } else {
          setSearchParams((prev) => ({
            ...prev,
            dep: dependency.project,
          }));
        }
      });
  }, [
    dependency,
    searchSourceBranch,
    searchSourceId,
    searchTargetBranch,
    searchTargetId,
    setSearchParams,
  ]);

  if (
    !searchSourceBranch ||
    !searchSourceId ||
    !searchTargetBranch ||
    !searchTargetId
  ) {
    return <Loading text="versions" />;
  }

  return (
    <VersionCompare
      projectSlug={dependency.project}
      sourceBranch={searchSourceBranch}
      targetBranch={searchTargetBranch}
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
