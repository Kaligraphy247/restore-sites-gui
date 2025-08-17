<script lang="ts">
    import { onMount } from "svelte";
    import { toast } from "svelte-sonner";
    import { formatUrl, formatDate } from "$lib/utils";
    import {
        saveCollection,
        type SiteEntry,
        type CollectionData,
        type CollectionRecord,
    } from "$lib/types";
    import { loadCollections } from "$lib/api/collections";
    import {
        Plus,
        Save,
        X,
        History,
        ExternalLink,
        Clock,
    } from "@lucide/svelte";

    let sites: string = $state("");
    let recentCollections: CollectionRecord[] = $state([]);
    let loadingRecent = $state(false);

    onMount(async () => {
        loadingRecent = true;
        try {
            const collections = await loadCollections();
            // Sort by updated_at (most recent first) and take first 3
            recentCollections = collections
                .sort(
                    (a, b) =>
                        new Date(b.updated_at).getTime() -
                        new Date(a.updated_at).getTime(),
                )
                .slice(0, 3);
        } catch (error) {
            console.error("Failed to load recent collections:", error);
        }
        loadingRecent = false;
    });

    async function saveSite() {
        if (sites.trim() === "") {
            toast.error("Please enter some URLs to save");
            return;
        }

        const formatted: SiteEntry[] = formatUrl(sites);
        console.log({ formatted });

        if (formatted.length === 0) {
            toast.error("No valid URLs found");
            return;
        }

        try {
            const result: CollectionData = await saveCollection(formatted, {
                browser: "Chrome",
                mode: "Incognito",
            });
            console.log("Collection saved:", result);
            toast.success(`Collection saved with ${formatted.length} sites`);
            sites = "";

            // Refresh recent collections after saving
            const collections = await loadCollections();
            recentCollections = collections
                .sort(
                    (a, b) =>
                        new Date(b.updated_at).getTime() -
                        new Date(a.updated_at).getTime(),
                )
                .slice(0, 3);
        } catch (error) {
            console.error("Save failed:", error);
            toast.error("Failed to save collection");
        }
    }

    function clearSites() {
        sites = "";
    }
</script>

<div class="space-y-6">
    <!-- Recent Collections Section -->
    <div class="space-y-4">
        <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
                <History
                    size={20}
                    class="text-neutral-600 dark:text-neutral-400"
                />
                <h2
                    class="font-semibold text-xl text-neutral-800 dark:text-neutral-100"
                >
                    Recent Collections
                </h2>
            </div>
            {#if recentCollections.length > 0}
                <a
                    href="/collections"
                    class="text-sm text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 flex items-center gap-1"
                >
                    View all
                    <ExternalLink size={14} />
                </a>
            {/if}
        </div>

        {#if loadingRecent}
            <div
                class="bg-neutral-50 dark:bg-neutral-800/50 border border-neutral-200 dark:border-neutral-700 rounded-lg p-6"
            >
                <div class="text-center text-neutral-500 dark:text-neutral-400">
                    <div
                        class="w-6 h-6 border-2 border-neutral-300 border-t-blue-600 rounded-full animate-spin mx-auto mb-2"
                    ></div>
                    <p class="text-sm">Loading recent collections...</p>
                </div>
            </div>
        {:else if recentCollections.length === 0}
            <div
                class="bg-neutral-50 dark:bg-neutral-800/50 border border-neutral-200 dark:border-neutral-700 rounded-lg p-6"
            >
                <div class="text-center text-neutral-500 dark:text-neutral-400">
                    <div
                        class="w-12 h-12 bg-neutral-200 dark:bg-neutral-700 rounded-full flex items-center justify-center mx-auto mb-3"
                    >
                        <History size={24} class="opacity-50" />
                    </div>
                    <p class="text-sm">
                        Your recent collections will appear here
                    </p>
                    <p class="text-xs mt-1">
                        Create your first collection below to get started
                    </p>
                </div>
            </div>
        {:else}
            <div class="grid gap-3">
                {#each recentCollections as collection (collection.id)}
                    <a href="/collections/{collection.id}" class="group">
                        <div
                            class="bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg p-4 hover:border-neutral-300 dark:hover:border-neutral-600 transition-colors duration-200"
                        >
                            <div class="flex items-start justify-between">
                                <div class="flex-1 min-w-0">
                                    <h3
                                        class="font-medium text-neutral-900 dark:text-neutral-100 truncate group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors duration-200"
                                    >
                                        {collection.name}
                                    </h3>
                                    <div
                                        class="flex items-center gap-4 mt-1 text-sm text-neutral-500 dark:text-neutral-400"
                                    >
                                        <span
                                            >{collection.sites.length} sites</span
                                        >
                                        <div class="flex items-center gap-1">
                                            <Clock size={12} />
                                            <span
                                                >{formatDate(
                                                    collection.updated_at,
                                                )}</span
                                            >
                                        </div>
                                    </div>
                                </div>
                                <ExternalLink
                                    size={16}
                                    class="text-neutral-400 group-hover:text-neutral-600 dark:group-hover:text-neutral-300 transition-colors duration-200 flex-shrink-0 mt-1"
                                />
                            </div>
                        </div>
                    </a>
                {/each}
            </div>
        {/if}
    </div>

    <!-- Add New Collection Section -->
    <div class="space-y-4">
        <div class="flex items-center gap-2">
            <Plus size={20} class="text-neutral-600 dark:text-neutral-400" />
            <h2
                class="font-semibold text-xl text-neutral-800 dark:text-neutral-100"
            >
                Create New Collection
            </h2>
        </div>

        <div
            class="bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg p-4 space-y-4"
        >
            <div class="space-y-2">
                <label
                    for="sites-input"
                    class="block text-sm font-medium text-neutral-700 dark:text-neutral-300"
                >
                    Paste your URLs and titles
                </label>
                <textarea
                    id="sites-input"
                    bind:value={sites}
                    name="projects"
                    placeholder="Paste your browser tabs here. Each line should be: Title https://example.com"
                    class="w-full h-[200px] border border-neutral-300 dark:border-neutral-600 rounded-lg px-4 py-3 resize-none bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 dark:placeholder:text-neutral-500 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                ></textarea>
                <div
                    class="flex items-center justify-between text-sm text-neutral-500 dark:text-neutral-400"
                >
                    <div>
                        {#if sites.trim()}
                            {formatUrl(sites).length} sites detected
                        {:else}
                            Enter URLs to create a collection
                        {/if}
                    </div>
                    <div class="text-xs">
                        Format: "Page Title https://example.com"
                    </div>
                </div>
            </div>

            <div class="flex justify-between items-center pt-2">
                <button
                    onclick={clearSites}
                    disabled={!sites.trim()}
                    class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-neutral-600 dark:text-neutral-400 bg-neutral-100 dark:bg-neutral-700 rounded-lg hover:bg-neutral-200 dark:hover:bg-neutral-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
                >
                    <X size={16} />
                    Clear
                </button>

                <button
                    onclick={saveSite}
                    disabled={!sites.trim()}
                    class="flex items-center gap-2 px-6 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 disabled:cursor-not-allowed rounded-lg transition-colors duration-200 shadow-sm"
                >
                    <Save size={16} />
                    Save Collection
                </button>
            </div>
        </div>
    </div>
</div>
