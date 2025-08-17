import type { PageLoad } from "./$types";
import { loadCollections, type CollectionRecord } from "$lib/types";

export const load: PageLoad = async ({}) => {
  const collections: Array<CollectionRecord> = await loadCollections();
  return { collections };
};
