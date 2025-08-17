// Typed Tauri API wrapper for browser profile operations

import { invoke } from "@tauri-apps/api/core";
import type { BrowserProfile, BrowserMode } from "$lib/types/models";

export class ProfileAPI {
  /**
   * Create a new browser profile
   */
  static async createBrowserProfile(profile: BrowserProfile): Promise<BrowserProfile> {
    try {
      const result = await invoke<BrowserProfile>("create_browser_profile", { profile });
      return result;
    } catch (error) {
      throw new Error(`Failed to create browser profile: ${error}`);
    }
  }

  /**
   * Get all browser profiles
   */
  static async getBrowserProfiles(): Promise<BrowserProfile[]> {
    try {
      const result = await invoke<BrowserProfile[]>("get_browser_profiles");
      return result;
    } catch (error) {
      throw new Error(`Failed to get browser profiles: ${error}`);
    }
  }

  /**
   * Get a single browser profile by ID
   */
  static async getBrowserProfile(id: string): Promise<BrowserProfile | null> {
    try {
      const result = await invoke<BrowserProfile | null>("get_browser_profile", { id });
      return result;
    } catch (error) {
      throw new Error(`Failed to get browser profile: ${error}`);
    }
  }

  /**
   * Update an existing browser profile
   */
  static async updateBrowserProfile(id: string, profile: BrowserProfile): Promise<BrowserProfile> {
    try {
      const result = await invoke<BrowserProfile>("update_browser_profile", { id, profile });
      return result;
    } catch (error) {
      throw new Error(`Failed to update browser profile: ${error}`);
    }
  }

  /**
   * Delete a browser profile by ID
   */
  static async deleteBrowserProfile(id: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>("delete_browser_profile", { id });
      return result;
    } catch (error) {
      throw new Error(`Failed to delete browser profile: ${error}`);
    }
  }

  /**
   * Check browser detection status for all profiles
   */
  static async checkBrowserDetection(): Promise<BrowserProfile[]> {
    try {
      const result = await invoke<BrowserProfile[]>("check_browser_detection");
      return result;
    } catch (error) {
      throw new Error(`Failed to check browser detection: ${error}`);
    }
  }

  /**
   * Get the global default browser mode
   */
  static async getDefaultBrowserMode(): Promise<BrowserMode> {
    try {
      const result = await invoke<BrowserMode>("get_default_browser_mode");
      return result;
    } catch (error) {
      throw new Error(`Failed to get default browser mode: ${error}`);
    }
  }

  /**
   * Set the global default browser mode
   */
  static async setDefaultBrowserMode(mode: BrowserMode): Promise<void> {
    try {
      await invoke<void>("set_default_browser_mode", { mode });
    } catch (error) {
      throw new Error(`Failed to set default browser mode: ${error}`);
    }
  }
}

// Convenience functions with better ergonomics
export async function getBrowserProfiles(): Promise<BrowserProfile[]> {
  return ProfileAPI.getBrowserProfiles();
}

export async function getBrowserProfile(id: string): Promise<BrowserProfile | null> {
  return ProfileAPI.getBrowserProfile(id);
}

export async function createBrowserProfile(profile: BrowserProfile): Promise<BrowserProfile> {
  return ProfileAPI.createBrowserProfile(profile);
}

export async function updateBrowserProfile(id: string, profile: BrowserProfile): Promise<BrowserProfile> {
  return ProfileAPI.updateBrowserProfile(id, profile);
}

export async function deleteBrowserProfile(id: string): Promise<boolean> {
  return ProfileAPI.deleteBrowserProfile(id);
}

export async function checkBrowserDetection(): Promise<BrowserProfile[]> {
  return ProfileAPI.checkBrowserDetection();
}

export async function getDefaultBrowserMode(): Promise<BrowserMode> {
  return ProfileAPI.getDefaultBrowserMode();
}

export async function setDefaultBrowserMode(mode: BrowserMode): Promise<void> {
  return ProfileAPI.setDefaultBrowserMode(mode);
}