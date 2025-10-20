import type { PageLoad } from "./$types";
import { getCollection } from "$lib/api/collections";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ params }) => {
  const id = parseInt(params.id);

  if (isNaN(id)) {
    throw error(400, "Invalid collection ID");
  }

  try {
    // Note: Tauri's invoke() uses window.fetch internally for IPC communication
    // This is required for desktop app functionality and cannot use SvelteKit's fetch
    const collection = await getCollection(id);

    if (!collection) {
      throw error(404, "Collection not found");
    }

    return {
      collection,
    };
  } catch (err) {
    console.error("Failed to load collection:", err);
    throw error(500, "Failed to load collection");
  }
};
