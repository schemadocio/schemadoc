import { AxiosInstance } from "axios";
import { Project, Dependency } from "../components/projects/models";
import { ApiResponse } from "./response";

const projectsApi = (axios: AxiosInstance) => ({
  list: () => axios.get<ApiResponse<Project[]>>("/v1/projects"),
  get: (slug: string) =>
    axios.get<ApiResponse<Project>>(`/v1/projects/${slug}`),
  dependents: (slug: string) =>
    axios.get<ApiResponse<Dependency[]>>(`/v1/projects/${slug}/dependents`),
});

export default projectsApi;
