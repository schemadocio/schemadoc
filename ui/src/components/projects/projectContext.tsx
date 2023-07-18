import React from "react";
import { Project } from "./models";

export const ProjectContext = React.createContext<Project | null>(null);
export default ProjectContext;
