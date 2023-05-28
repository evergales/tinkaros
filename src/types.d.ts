type ID = string;
type Number = number;

interface CurseForgeMod {
  id: ID;
  game_id: ID;
  name: string;
  slug: string;
  links: { websiteUrl: string };
  summary: string;
  status: unknown;
  download_count: Number;
  is_featured: boolean;
  primary_category_id: ID;
  categories: unknown[];
  class_id?: Number;
  authors: unknown[];
  logo?: { url: string };
  screenshots: unknown[];
  main_file_id: ID;
  latest_files: unknown[];
  latest_files_indexes: unknown[];
  date_created: unknown;
  date_modified: unknown;
  date_released: unknown;
  allow_mod_distribution?: boolean;
  game_popularity_rank: Number;
  is_available: boolean;
  thumbs_up_count?: Number;
  latest_early_access_files_indexes?: string[];
}

interface ModrinthProject {
  slug: string;
  title: string;
  description: string;
  categories: string[];
  client_side: unknown;
  server_side: unknown;
  body: string;
  additional_categories: string[];
  issues_url?: unknown;
  source_url?: unknown;
  wiki_url?: unknown;
  discord_url?: unknown;
  donation_urls: unknown[];
  project_type: unknown;
  downloads: Number;
  icon_url?: string;
  color?: Number;
  id: ID;
  team: ID;
  body_url?: unknown;
  moderator_message?: unknown;
  published: unknown;
  updated: unknown;
  approved?: unknown;
  followers: Number;
  status: unknown;
  license: unknown;
  versions: ID[];
  game_versions: string[];
  loaders: string[];
  gallery: unknown[];
}

interface Mod  {
  ModrinthProject: ModrinthProject | undefined
  CurseForgeMod: CurseForgeMod | undefined
}