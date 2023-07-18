export interface Project {
  slug: string;
  name: string;
  description: string;

  alerts: Alert[] | null;
  dataSource: DataSource | null;
}

export interface Alert {
  name: string;
  service: string;

  isActive: boolean;

  source: "own" | "deps";

  kind: "all" | "all-breaking";
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
