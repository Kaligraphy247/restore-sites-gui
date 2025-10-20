<script lang="ts">
    import { X, RefreshCw, Plus } from "@lucide/svelte";
    import type { SiteEntry } from "$lib/types/models";
    import { formatUrl } from "$lib/utils";
    import { toast } from "svelte-sonner";

    interface Props {
        show: boolean;
        currentSites: SiteEntry[];
        onUpdate: (newSites: SiteEntry[], refreshMode: boolean) => Promise<void>;
        onCancel: () => void;
    }

    let {
        show,
        currentSites,
        onUpdate,
        onCancel
    }: Props = $props();

    let sitesText = $state("");
    let parsedSites = $state<SiteEntry[]>([]);
    let refreshMode = $state(false);

    // Parse sites from text whenever sitesText changes
    $effect(() => {
        if (sitesText.trim()) {
            const parsed = formatUrl(sitesText);
            parsedSites = parsed;
        } else {
            parsedSites = [];
        }
    });

    // Reset form when modal opens/closes
    $effect(() => {
        if (!show) {
            sitesText = "";
            parsedSites = [];
            refreshMode = false;
        }
    });

    function clearSites() {
        sitesText = "";
        parsedSites = [];
    }

    // Calculate how many sites will be added/updated
    let updateStats = $derived.by(() => {
        if (parsedSites.length === 0) {
            return { new: 0, total: 0, duplicates: 0 };
        }

        if (refreshMode) {
            // In refresh mode, only add truly new sites (title+url not in current)
            const currentSet = new Set(
                currentSites.map(s => `${s.title}|||${s.url}`)
            );
            const newCount = parsedSites.filter(
                s => !currentSet.has(`${s.title}|||${s.url}`)
            ).length;
            const duplicateCount = parsedSites.length - newCount;
            return { new: newCount, total: parsedSites.length, duplicates: duplicateCount };
        } else {
            // In add mode, all parsed sites will be added
            return { new: parsedSites.length, total: parsedSites.length, duplicates: 0 };
        }
    });

    async function handleUpdate() {
        if (parsedSites.length === 0) {
            toast.error("Please paste some URLs to update the collection");
            return;
        }

        await onUpdate(parsedSites, refreshMode);
    }
</script>

{#if show}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50">
        <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-hidden flex flex-col">
            <!-- Header -->
            <div class="flex items-center justify-between p-4 border-b border-neutral-200 dark:border-neutral-700">
                <h3 class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
                    Update Collection
                </h3>
                <button
                    onclick={onCancel}
                    class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
                    aria-label="Close dialog"
                >
                    <X class="w-6 h-6" />
                </button>
            </div>

            <!-- Content -->
            <div class="flex-1 overflow-y-auto p-4 space-y-4">
                <!-- Info Banner -->
                <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
                    <div class="flex items-start gap-3">
                        <div class="w-5 h-5 bg-blue-100 dark:bg-blue-900/40 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                            <RefreshCw size={12} class="text-blue-600 dark:text-blue-400" />
                        </div>
                        <div>
                            <p class="text-sm text-blue-800 dark:text-blue-200 font-medium">
                                Bulk Update Mode
                            </p>
                            <p class="text-xs text-blue-700 dark:text-blue-300 mt-1">
                                Paste URLs to add or refresh sites in this collection. Use the refresh checkbox to control behavior.
                            </p>
                        </div>
                    </div>
                </div>

                <!-- Paste URLs Section -->
                <div>
                    <label for="sites-text" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                        Paste URLs and Titles
                    </label>
                    <textarea
                        id="sites-text"
                        bind:value={sitesText}
                        placeholder="Paste your browser tabs here. Each line should be: Title https://example.com"
                        class="w-full h-40 border border-neutral-300 dark:border-neutral-600 rounded-md px-3 py-2 resize-none bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 dark:placeholder:text-neutral-500 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm"
                    ></textarea>
                    <div class="flex items-center justify-between text-xs text-neutral-500 dark:text-neutral-400 mt-1">
                        <span>
                            {#if sitesText.trim()}
                                {parsedSites.length} sites detected
                            {:else}
                                Enter URLs to update the collection
                            {/if}
                        </span>
                        <div class="flex items-center gap-2">
                            <span>Format: "Title https://example.com"</span>
                            {#if sitesText.trim()}
                                <button
                                    onclick={clearSites}
                                    class="text-blue-600 dark:text-blue-400 hover:underline"
                                >
                                    Clear
                                </button>
                            {/if}
                        </div>
                    </div>
                </div>

                <!-- Refresh Mode Checkbox -->
                <div class="bg-neutral-50 dark:bg-neutral-700 rounded-lg p-4 border border-neutral-200 dark:border-neutral-600">
                    <div class="flex items-start gap-3">
                        <input
                            type="checkbox"
                            id="refresh-mode"
                            bind:checked={refreshMode}
                            class="w-4 h-4 mt-0.5 text-blue-600 bg-white dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 rounded focus:ring-blue-500"
                        />
                        <div class="flex-1">
                            <label
                                for="refresh-mode"
                                class="text-sm font-medium text-neutral-700 dark:text-neutral-300 cursor-pointer"
                            >
                                Refresh mode (only add new sites)
                            </label>
                            <p class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">
                                {#if refreshMode}
                                    ✓ Only sites with unique title+URL combinations will be added. Existing sites remain unchanged.
                                {:else}
                                    All pasted sites will be added to the collection, even if duplicates exist.
                                {/if}
                            </p>
                        </div>
                    </div>
                </div>

                <!-- Preview -->
                {#if parsedSites.length > 0}
                    <div class="bg-neutral-50 dark:bg-neutral-700 rounded-lg p-4 border border-neutral-200 dark:border-neutral-600">
                        <h4 class="text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">
                            Preview ({parsedSites.length} sites)
                        </h4>
                        <div class="space-y-2 max-h-64 overflow-y-auto">
                            {#each parsedSites as site, index (index)}
                                <div class="bg-white dark:bg-neutral-800 rounded p-2 text-xs">
                                    <div class="font-medium text-neutral-900 dark:text-neutral-100 truncate">
                                        {site.title}
                                    </div>
                                    <div class="text-neutral-500 dark:text-neutral-400 truncate">
                                        {site.url}
                                    </div>
                                </div>
                            {/each}
                        </div>

                        <!-- Stats -->
                        <div class="mt-3 pt-3 border-t border-neutral-200 dark:border-neutral-600 text-xs space-y-1">
                            {#if refreshMode}
                                <div class="flex items-center justify-between">
                                    <span class="text-neutral-600 dark:text-neutral-400">
                                        Total sites parsed:
                                    </span>
                                    <span class="font-medium text-neutral-900 dark:text-neutral-100">
                                        {updateStats.total}
                                    </span>
                                </div>
                                <div class="flex items-center justify-between">
                                    <span class="text-green-600 dark:text-green-400">
                                        <Plus class="w-3 h-3 inline mr-1" />
                                        New sites to add:
                                    </span>
                                    <span class="font-medium text-green-600 dark:text-green-400">
                                        {updateStats.new}
                                    </span>
                                </div>
                                {#if updateStats.duplicates > 0}
                                    <div class="flex items-center justify-between">
                                        <span class="text-neutral-500 dark:text-neutral-500">
                                            Duplicates (skipped):
                                        </span>
                                        <span class="font-medium text-neutral-500 dark:text-neutral-500">
                                            {updateStats.duplicates}
                                        </span>
                                    </div>
                                {/if}
                            {:else}
                                <div class="flex items-center justify-between">
                                    <span class="text-neutral-600 dark:text-neutral-400">
                                        <Plus class="w-3 h-3 inline mr-1" />
                                        Sites to add:
                                    </span>
                                    <span class="font-medium text-neutral-900 dark:text-neutral-100">
                                        {updateStats.new}
                                    </span>
                                </div>
                                <p class="text-neutral-500 dark:text-neutral-400 italic">
                                    All sites will be added (duplicates allowed)
                                </p>
                            {/if}
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer -->
            <div class="flex justify-end gap-3 p-4 border-t border-neutral-200 dark:border-neutral-700">
                <button
                    onclick={onCancel}
                    class="px-4 py-2 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded-md transition-colors"
                >
                    Cancel
                </button>
                <button
                    onclick={handleUpdate}
                    disabled={parsedSites.length === 0}
                    class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white hover:bg-blue-700 disabled:bg-blue-400 disabled:cursor-not-allowed rounded-md transition-colors"
                >
                    <RefreshCw class="w-4 h-4" />
                    {refreshMode ? 'Refresh Collection' : 'Add to Collection'}
                </button>
            </div>
        </div>
    </div>
{/if}
