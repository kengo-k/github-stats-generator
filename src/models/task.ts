export interface Task {
  id: number;
  name: string;
  desc: string;
}

export interface Project {
  repository_name: string;
  commit_count: number;
}

export interface ActiveProjects {
  from_date: string;
  to_date: string;
  projects: (Project & {
    ratio: number;
  })[];
}
