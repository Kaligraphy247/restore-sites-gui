// Typed Tauri API wrapper for collection operations

import { invoke } from "@tauri-apps/api/core";
import type {
  CollectionData,
  SaveCollectionRequest,
  SiteEntry,
  CollectionConfig,
  CollectionRecord,
} from "$lib/types/models";
import { DEFAULT_COLLECTION_CONFIG } from "$lib/types/models";

export class CollectionAPI {
  /**
   * Save a new collection to the database
   */
  static async saveCollection(
    request: SaveCollectionRequest,
  ): Promise<CollectionData> {
    try {
      const result = await invoke<CollectionData>("save_collection", {
        request: {
          sites: request.sites,
          config: request.config || DEFAULT_COLLECTION_CONFIG,
        },
      });
      return result;
    } catch (error) {
      throw new Error(`Failed to save collection: ${error}`);
    }
  }

  /**
   * Load all collections from the database
   */
  static async loadCollections(): Promise<CollectionRecord[]> {
    try {
      const result = await invoke<CollectionRecord[]>("load_collections");
      return result;
    } catch (error) {
      throw new Error(`Failed to load collections: ${error}`);
    }
  }

  /**
   * Restore a collection by opening all URLs in the configured browser
   */
  static async restoreCollection(sites: SiteEntry[]): Promise<void> {
    try {
      await invoke<void>("restore_collection", { sites });
    } catch (error) {
      throw new Error(`Failed to restore collection: ${error}`);
    }
  }

  /**
   * Get a single collection by ID
   */
  static async getCollection(id: number): Promise<CollectionRecord | null> {
    try {
      const result = await invoke<CollectionRecord | null>("get_collection", { id });
      return result;
    } catch (error) {
      throw new Error(`Failed to get collection: ${error}`);
    }
  }

  /**
   * Update an existing collection
   */
  static async updateCollection(
    id: number,
    collectionData: CollectionData,
  ): Promise<CollectionRecord> {
    try {
      const result = await invoke<CollectionRecord>("update_collection", {
        id,
        collectionData,
      });
      return result;
    } catch (error) {
      throw new Error(`Failed to update collection: ${error}`);
    }
  }

  /**
   * Delete a collection by ID
   */
  static async deleteCollection(id: number): Promise<boolean> {
    try {
      const result = await invoke<boolean>("delete_collection", { id });
      return result;
    } catch (error) {
      throw new Error(`Failed to delete collection: ${error}`);
    }
  }

  /**
   * Test greeting command (can be removed later)
   */
  static async greet(name: string): Promise<string> {
    try {
      return await invoke<string>("greet", { name });
    } catch (error) {
      throw new Error(`Failed to greet: ${error}`);
    }
  }
}

// Convenience functions with better ergonomics
export async function saveCollection(
  sites: SiteEntry[],
  config?: Partial<CollectionConfig>,
): Promise<CollectionData> {
  const fullConfig: CollectionConfig = {
    browser_profile_id: config?.browser_profile_id,
    browser: config?.browser,
    mode: config?.mode,
    custom_path: config?.custom_path,
  };

  return CollectionAPI.saveCollection({
    sites,
    config: fullConfig,
  });
}

export async function loadCollections(): Promise<CollectionRecord[]> {
  return CollectionAPI.loadCollections();
}

export async function restoreCollection(sites: SiteEntry[]): Promise<void> {
  return CollectionAPI.restoreCollection(sites);
}

export async function getCollection(id: number): Promise<CollectionRecord | null> {
  return CollectionAPI.getCollection(id);
}

export async function updateCollection(
  id: number,
  sites: SiteEntry[],
  name?: string,
  config?: Partial<CollectionConfig>,
): Promise<CollectionRecord> {
  const fullConfig: CollectionConfig = {
    browser_profile_id: config?.browser_profile_id,
    browser: config?.browser,
    mode: config?.mode,
    custom_path: config?.custom_path,
  };

  const collectionData: CollectionData = {
    sites,
    name,
    config: fullConfig,
    created_at: new Date().toISOString(), // This will be overridden by the service
  };

  return CollectionAPI.updateCollection(id, collectionData);
}

export async function deleteCollection(id: number): Promise<boolean> {
  return CollectionAPI.deleteCollection(id);
}
