import { AxiosInstance } from "axios";
import { Project } from "../components/projects/models";

const projectsApi = (axios: AxiosInstance) => ({
  list: () => axios.get<Project[]>("/v1/projects"),
  get: (slug: string) => axios.get<Project>(`/v1/projects/${slug}`),
});

export default projectsApi;
