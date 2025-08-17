// TypeScript types that mirror Rust models for full type safety

export interface SiteEntry {
  title: string;
  url: string;
}

export interface SaveCollectionRequest {
  sites: SiteEntry[];
  config?: CollectionConfig;
}

export interface CollectionData {
  sites: SiteEntry[];
  created_at: string; // ISO 8601 DateTime string
  name?: string;
  config: CollectionConfig;
}

export interface CollectionConfig {
  browser: Browser;
  mode: BrowserMode;
  custom_path?: string;
}

export type Browser = 
  | "Chrome" 
  | "Firefox" 
  | "Safari" 
  | "Edge" 
  | { Custom: string };

export type BrowserMode = 
  | "Normal" 
  | "Incognito" 
  | "Private";

// JSON Database Schema Types
export interface DatabaseMeta {
  last_updated_id: number;
  last_updated: string; // ISO 8601 DateTime string
  max_id: number;
  record_count: number;
  created_at: string; // ISO 8601 DateTime string
}

export interface Database {
  meta: DatabaseMeta;
  data: CollectionRecord[];
}

export interface CollectionRecord {
  id: number;
  name: string;
  sites: SiteEntry[];
  config: CollectionConfig;
  created_at: string; // ISO 8601 DateTime string
  updated_at: string; // ISO 8601 DateTime string
}

// Helper types for frontend use
export interface CollectionFormData {
  name?: string;
  sites: SiteEntry[];
  browser?: Browser;
  mode?: BrowserMode;
  custom_path?: string;
}

// Type guards for runtime validation
export function isBrowser(value: unknown): value is Browser {
  if (typeof value === "string") {
    return ["Chrome", "Firefox", "Safari", "Edge"].includes(value);
  }
  if (typeof value === "object" && value !== null && "Custom" in value) {
    return typeof (value as any).Custom === "string";
  }
  return false;
}

export function isBrowserMode(value: unknown): value is BrowserMode {
  return typeof value === "string" && 
         ["Normal", "Incognito", "Private"].includes(value);
}

export function isSiteEntry(value: unknown): value is SiteEntry {
  return typeof value === "object" && 
         value !== null && 
         typeof (value as any).title === "string" && 
         typeof (value as any).url === "string";
}

// Default values that match Rust defaults
export const DEFAULT_COLLECTION_CONFIG: CollectionConfig = {
  browser: "Chrome",
  mode: "Incognito",
  custom_path: undefined,
};

// Utility functions
export function createCollectionConfig(
  browser: Browser = "Chrome",
  mode: BrowserMode = "Incognito",
  custom_path?: string
): CollectionConfig {
  return {
    browser,
    mode,
    custom_path,
  };
}

export function createSaveCollectionRequest(
  sites: SiteEntry[],
  config: CollectionConfig = DEFAULT_COLLECTION_CONFIG
): SaveCollectionRequest & { config: CollectionConfig } {
  return {
    sites,
    config,
  };
}