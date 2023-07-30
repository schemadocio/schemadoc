export interface Project {
  slug: string;
  name: string;
  kind: "server" | "client";
  description: string;

  links: Link[] | null;

  alerts: Alert[] | null;
  dataSource: DataSource | null;
  dependencies: Dependency[] | null;
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
  source: DataSourceSource;
  status: DataSourceStatus | null;
}
