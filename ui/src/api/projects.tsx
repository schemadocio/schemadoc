import { AxiosInstance } from "axios";
import { Project, Dependency } from "../components/projects/models";

const projectsApi = (axios: AxiosInstance) => ({
  list: () => axios.get<Project[]>("/v1/projects"),
  get: (slug: string) => axios.get<Project>(`/v1/projects/${slug}`),
  dependents: (slug: string) =>
    axios.get<Dependency[]>(`/v1/projects/${slug}/dependents`),
});

export default projectsApi;
