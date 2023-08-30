import { AxiosInstance } from "axios";
import { HttpSchemaDiff } from "../components/http-schema/models";
import { Version, VersionStatistics } from "../components/versions/models";
import { ApiResponse } from "./response";

const versionsApi = (axios: AxiosInstance) => ({
  list: (projectSlug: string, branchName: string) =>
    axios.get<ApiResponse<Version[]>>(
      `/v1/projects/${projectSlug}/branches/${encodeURI(branchName)}/versions`
    ),
  get: (projectSlug: string, branchName: string, id: number) =>
    axios.get<ApiResponse<Version>>(
      `/v1/projects/${projectSlug}/branches/${encodeURI(
        branchName
      )}/versions/${id}`
    ),
  add: (projectSlug: string, branchName: string, id: number) =>
    axios.post<ApiResponse<Version | null>>(
      `/v1/projects/${projectSlug}/branches/${encodeURI(
        branchName
      )}/versions/${id}`
    ),
  getDiff: (projectSlug: string, branchName: string, id: number) =>
    axios.get<HttpSchemaDiff | null>(
      `/v1/projects/${projectSlug}/branches/${encodeURI(
        branchName
      )}/versions/${id}/diff`
    ),
  compare: (
    projectSlug: string,
    srcBranch: string,
    srcId: number,
    tgtBranch: string,
    tgtId: number
  ) =>
    axios.get<
      ApiResponse<{
        diff: HttpSchemaDiff;
        statistics: VersionStatistics;
      } | null>
    >(
      `/v1/projects/${projectSlug}/branches/${encodeURI(
        srcBranch
      )}/versions/${srcId}/compare/${encodeURI(tgtBranch)}/${tgtId}`
    ),
});

export default versionsApi;
