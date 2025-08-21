<script lang="ts">
    import type { PageLoad, PageProps } from "./$types";
    import type { CollectionRecord } from "$lib/types/models";
    import {
        ArchiveRestore,
        ExternalLink,
        Calendar,
        Globe,
        Search,
        X,
    } from "@lucide/svelte";
    import { restoreCollection } from "$lib/types";
    import Fuse from "fuse.js";

    let { data }: PageProps = $props();
    console.log(data.collections);

    // Search state
    let searchQuery = $state("");
    let searchInput: HTMLInputElement;

    // Fuse.js configuration
    const fuseOptions = {
        keys: [
            { name: 'sites.url', weight: 0.4 },     // 40% weight for URLs
            { name: 'sites.title', weight: 0.4 },   // 40% weight for titles
            { name: 'name', weight: 0.2 }           // 20% weight for collection name
        ],
        threshold: 0.4, // Allow some fuzziness but not too much
        includeScore: true,
        minMatchCharLength: 2,
        ignoreLocation: true // Don't factor in location of match
    };

    // Initialize Fuse instance
    const fuse = new Fuse(data.collections, fuseOptions);

    // Filtered collections based on search
    let filteredCollections = $derived.by(() => {
        if (!searchQuery.trim()) {
            return data.collections;
        }
        
        const results = fuse.search(searchQuery.trim());
        return results.map(result => result.item);
    });

    async function handleRestore(sites: any[]) {
        try {
            await restoreCollection(sites);
        } catch (error) {
            console.error("Failed to restore collection:", error);
        }
    }

    function clearSearch() {
        searchQuery = "";
        searchInput?.focus();
    }
</script>

<div class="space-y-6">
    <div class="flex items-center justify-between">
        <h2
            class="font-semibold text-2xl text-neutral-800 dark:text-neutral-100"
        >
            Collections
        </h2>
        <div class="text-sm text-neutral-500 dark:text-neutral-400">
            {searchQuery ? filteredCollections.length : data.collections.length} collection{(searchQuery ? filteredCollections.length : data.collections.length) !== 1
                ? "s"
                : ""}{searchQuery ? ` (${data.collections.length} total)` : ""}
        </div>
    </div>

    <!-- Search Input -->
    <div class="relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Search size={16} class="text-neutral-400 dark:text-neutral-500" />
        </div>
        <input
            bind:this={searchInput}
            bind:value={searchQuery}
            type="text"
            placeholder="Search collections, titles, or URLs..."
            class="w-full pl-10 pr-10 py-2.5 border border-neutral-200 dark:border-neutral-700 rounded-lg bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 placeholder-neutral-500 dark:placeholder-neutral-400 focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent transition-colors"
        />
        {#if searchQuery}
            <button
                onclick={clearSearch}
                class="absolute inset-y-0 right-0 pr-3 flex items-center text-neutral-400 dark:text-neutral-500 hover:text-neutral-600 dark:hover:text-neutral-300 transition-colors"
                title="Clear search"
            >
                <X size={16} />
            </button>
        {/if}
    </div>

    <div class="space-y-3 h-[450px] overflow-y-auto pr-4 pb-8 -mr-2">
        {#each filteredCollections as collection}
            <div
                class="group bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-sm hover:shadow-md transition-all duration-200"
            >
                <div class="p-4">
                    <!-- Header -->
                    <div class="flex items-start justify-between mb-3">
                        <div class="flex-1 min-w-0">
                            <h3
                                class="font-medium text-neutral-900 dark:text-neutral-100 truncate"
                            >
                                {collection.name}
                            </h3>
                            <div
                                class="flex items-center gap-4 mt-1 text-sm text-neutral-500 dark:text-neutral-400"
                            >
                                <div class="flex items-center gap-1 truncate">
                                    <Calendar size={14} />
                                    {new Date(
                                        collection.created_at,
                                    ).toDateString()} @ {new Date(
                                        collection.created_at,
                                    ).toLocaleTimeString()}
                                </div>
                                <div class="flex items-center gap-1">
                                    <Globe size={14} />
                                    {collection.sites.length} site{collection
                                        .sites.length !== 1
                                        ? "s"
                                        : ""}
                                </div>
                            </div>
                        </div>
                        <div class="flex items-center gap-2 ml-4">
                            <button
                                onclick={() => handleRestore(collection.sites)}
                                class="flex items-center gap-1 px-3 py-1.5 text-sm bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors duration-200"
                                title="Restore collection"
                            >
                                <ArchiveRestore size={14} />
                                Restore
                            </button>
                            <a
                                href={`/collections/${collection.id}`}
                                class="flex items-center gap-1 px-3 py-1.5 text-sm bg-neutral-50 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-600 transition-colors duration-200"
                                title="View details"
                            >
                                <ExternalLink size={14} />
                                View
                            </a>
                        </div>
                    </div>

                    <!-- Sites Preview -->
                    <div class="space-y-1.5">
                        {#each collection.sites.slice(0, 3) as site}
                            <div
                                class="flex items-center gap-2 text-sm text-neutral-600 dark:text-neutral-300 bg-neutral-50 dark:bg-neutral-700/50 rounded px-2 py-1.5"
                            >
                                <div
                                    class="w-2 h-2 bg-neutral-400 dark:bg-neutral-500 rounded-full flex-shrink-0"
                                ></div>
                                <span
                                    class="font-medium truncate flex-1"
                                    title={site.url}>{site.title}</span
                                >
                                <span
                                    class="text-xs text-neutral-400 dark:text-neutral-500 truncate max-w-[120px]"
                                >
                                    {site.url
                                        .replace(/^https?:\/\//, "")
                                        .split("/")[0]}
                                </span>
                            </div>
                        {/each}

                        {#if collection.sites.length > 3}
                            <div
                                class="text-xs text-neutral-400 dark:text-neutral-500 text-center py-1"
                            >
                                +{collection.sites.length - 3} more sites...
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        {/each}

        {#if filteredCollections.length === 0}
            <div
                class="text-center py-12 text-neutral-500 dark:text-neutral-400"
            >
                {#if searchQuery}
                    <Search size={48} class="mx-auto mb-4 opacity-50" />
                    <h3 class="font-medium mb-2">No matching collections</h3>
                    <p class="text-sm mb-4">
                        No collections found for "{searchQuery}"
                    </p>
                    <button
                        onclick={clearSearch}
                        class="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm font-medium transition-colors"
                    >
                        Clear search
                    </button>
                {:else}
                    <Globe size={48} class="mx-auto mb-4 opacity-50" />
                    <h3 class="font-medium mb-2">No collections yet</h3>
                    <p class="text-sm">
                        Create your first collection from the home page
                    </p>
                {/if}
            </div>
        {/if}
    </div>
</div>
