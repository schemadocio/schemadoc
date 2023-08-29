export interface Project {
  slug: string;
  name: string;
  kind: "server" | "client";
  description: string;

  links: Link[];
  alerts: Alert[];

  branches: string[];

  dataSources: DataSource[];
  dependencies: Dependency[];
}

export interface Link {
  name: string;
  url: string;
}

export interface Alert {
  name: string;
  service: string;

  isActive: boolean;

  source: "own" | "deps";

  kind: "all" | "breaking";
}

export interface Dependency {
  branch: string;
  project: string;
  version: number;
  outdated: boolean;
  breaking: boolean;
}

export interface DataSourceStatus {
  pullEnabled: boolean;
  pullAttempt: number;
  pullIntervalMinutes: number;
  pullLastAt: string;
  pullError: boolean;
}

export interface DataSourceSource {
  Url?: { url: string };
}

export interface DataSource {
  name: string;
  branch: string;
  source: DataSourceSource;
  status: DataSourceStatus | null;
}
