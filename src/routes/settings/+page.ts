import type { PageLoad } from "./$types";
import { getBrowserProfiles, getDefaultBrowserMode } from "$lib/api/profiles";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async () => {
  try {
    // Load profiles and default mode in parallel
    const [profiles, defaultMode] = await Promise.all([
      getBrowserProfiles(),
      getDefaultBrowserMode(),
    ]);

    return {
      profiles,
      defaultMode,
    };
  } catch (err) {
    console.error("Failed to load settings data:", err);
    throw error(500, "Failed to load settings data");
  }
};