import { AxiosInstance } from "axios";
import { Version } from "../components/versions/models";
import { HttpSchemaDiff } from "../components/http-schema/models";

const versionsApi = (axios: AxiosInstance) => ({
  list: (projectSlug: string) =>
    axios.get<Version[]>(`/v1/projects/${projectSlug}/versions`),
  get: (projectSlug: string, id: number) =>
    axios.get<Version>(`/v1/projects/${projectSlug}/versions/${id}`),
  add: (projectSlug: string, id: number) =>
    axios.post<Version | null>(`/v1/projects/${projectSlug}/versions/${id}`),
  getDiff: (projectSlug: string, id: number) =>
    axios.get<HttpSchemaDiff | null>(
      `/v1/projects/${projectSlug}/versions/${id}/diff`
    ),
  compare: (projectSlug: string, srcId: number, tgtId: number) =>
    axios.get<HttpSchemaDiff | null>(
      `/v1/projects/${projectSlug}/versions/${srcId}/compare/${tgtId}`
    ),
});

export default versionsApi;
