export interface Version {
  id: number;
  version: string;
  message: string;
  filePath: string;
  diffFilePath: string;
  createdAt: string;
  statistics: VersionStatistics;
}

export interface VersionStatistics {
  total: number;
  added: number;
  updated: number;
  removed: number;
}
