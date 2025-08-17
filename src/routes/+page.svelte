<script lang="ts">
    import { toast } from "svelte-sonner";
    import { formatUrl } from "$lib/utils";
    import { saveCollection, type SiteEntry, type CollectionData } from "$lib/types";
    import { Plus, Save, X, History } from "@lucide/svelte";

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
                mode: "Incognito"
            });
            console.log("Collection saved:", result);
            toast.success(`Collection saved with ${formatted.length} sites`);
            sites = "";
        } catch (error) {
            console.error("Save failed:", error);
            toast.error("Failed to save collection");
        }
    }

    function clearSites() {
        sites = "";
    }

    let sites: string = $state("");
</script>

<div class="space-y-6">
    <!-- Recent Collections Section -->
    <div class="space-y-4">
        <div class="flex items-center gap-2">
            <History size={20} class="text-neutral-600 dark:text-neutral-400" />
            <h2 class="font-semibold text-xl text-neutral-800 dark:text-neutral-100">Recent Collections</h2>
        </div>
        <div class="bg-neutral-50 dark:bg-neutral-800/50 border border-neutral-200 dark:border-neutral-700 rounded-lg p-6">
            <div class="text-center text-neutral-500 dark:text-neutral-400">
                <div class="w-12 h-12 bg-neutral-200 dark:bg-neutral-700 rounded-full flex items-center justify-center mx-auto mb-3">
                    <History size={24} class="opacity-50" />
                </div>
                <p class="text-sm">Your recent collections will appear here</p>
            </div>
        </div>
    </div>

    <!-- Add New Collection Section -->
    <div class="space-y-4">
        <div class="flex items-center gap-2">
            <Plus size={20} class="text-neutral-600 dark:text-neutral-400" />
            <h2 class="font-semibold text-xl text-neutral-800 dark:text-neutral-100">Create New Collection</h2>
        </div>
        
        <div class="bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg p-6 space-y-4">
            <div class="space-y-2">
                <label for="sites-input" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300">
                    Paste your URLs and titles
                </label>
                <textarea
                    id="sites-input"
                    bind:value={sites}
                    name="projects"
                    placeholder="Paste your browser tabs here. Each line should be: Title https://example.com"
                    class="w-full border border-neutral-300 dark:border-neutral-600 rounded-lg h-[200px] px-4 py-3 resize-none bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 dark:placeholder:text-neutral-500 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                />
                <div class="flex items-center justify-between text-sm text-neutral-500 dark:text-neutral-400">
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
