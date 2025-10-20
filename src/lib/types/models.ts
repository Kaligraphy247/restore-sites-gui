// TypeScript types that mirror Rust models for full type safety

export interface SiteEntry {
  title: string;
  url: string;
}

export interface SaveCollectionRequest {
  sites: SiteEntry[];
  config?: CollectionConfig;
  name?: string;
}

export interface CollectionData {
  sites: SiteEntry[];
  created_at: string; // ISO 8601 DateTime string
  name?: string;
  config: CollectionConfig;
}

export interface CollectionConfig {
  browser_profile_id?: string;  // PRIMARY: Reference to profile
  // Fallback fields for direct config
  browser?: Browser;
  mode?: BrowserMode;
  custom_path?: string;
}

export type Browser =
  | "Chrome"
  | "Firefox"
  | "Safari"
  | "Edge"
  | { Custom: string };

export type BrowserMode = "Normal" | "Incognito" | "Private";

// Browser Profile Type
export interface BrowserProfile {
  id: string;                    // "default-chrome", "work-firefox-1"
  name: string;                  // Max 64 UTF-8 chars, user-editable
  browser: Browser;              // Chrome | Firefox | Safari | Edge | Custom
  mode: BrowserMode;             // Normal | Incognito | Private
  custom_path?: string;          // Optional custom browser path
  is_default: boolean;           // One profile marked as system default
  is_detected: boolean;          // Currently detected on system
  created_at: string;            // ISO 8601 DateTime string
  updated_at: string;            // ISO 8601 DateTime string
}

// JSON Database Schema Types
export interface DatabaseMeta {
  version: number;               // Schema version (v2 for profiles)
  last_updated_id: number;
  last_updated: string; // ISO 8601 DateTime string
  max_id: number;
  record_count: number;
  created_at: string; // ISO 8601 DateTime string
  default_browser_mode: BrowserMode;  // Global default mode
}

export interface Database {
  meta: DatabaseMeta;
  profiles: BrowserProfile[];    // Browser profiles array
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
  return (
    typeof value === "string" &&
    ["Normal", "Incognito", "Private"].includes(value)
  );
}

export function isSiteEntry(value: unknown): value is SiteEntry {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as any).title === "string" &&
    typeof (value as any).url === "string"
  );
}

export function isBrowserProfile(value: unknown): value is BrowserProfile {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as any).id === "string" &&
    typeof (value as any).name === "string" &&
    isBrowser((value as any).browser) &&
    isBrowserMode((value as any).mode) &&
    typeof (value as any).is_default === "boolean" &&
    typeof (value as any).is_detected === "boolean" &&
    typeof (value as any).created_at === "string" &&
    typeof (value as any).updated_at === "string"
  );
}

// Default values that match Rust defaults
export const DEFAULT_COLLECTION_CONFIG: CollectionConfig = {
  browser_profile_id: undefined,  // No profile by default
  browser: undefined,             // No direct config by default
  mode: undefined,
  custom_path: undefined,
};

// Utility functions
export function createCollectionConfig(
  browser?: Browser,
  mode?: BrowserMode,
  custom_path?: string,
  browser_profile_id?: string,
): CollectionConfig {
  return {
    browser_profile_id,
    browser,
    mode,
    custom_path,
  };
}

export function createSaveCollectionRequest(
  sites: SiteEntry[],
  config: CollectionConfig = DEFAULT_COLLECTION_CONFIG,
): SaveCollectionRequest & { config: CollectionConfig } {
  return {
    sites,
    config,
  };
}
