<script lang="ts">
    import { Save, X, Plus, Trash2, PencilLine, Check } from "@lucide/svelte";
    import type { SiteEntry } from "$lib/types/models";
    import { toast } from "svelte-sonner";

    interface Props {
        show: boolean;
        sites: SiteEntry[];
        collectionName: string;
        onSave: (name: string, sites: SiteEntry[]) => Promise<void>;
        onCancel: () => void;
        onNameChange: (name: string) => void;
    }

    let {
        show,
        sites: initialSites,
        collectionName,
        onSave,
        onCancel,
        onNameChange
    }: Props = $props();

    let sites = $state([...initialSites]);
    let editingIndex: number | null = $state(null);
    let editTitle = $state("");
    let editUrl = $state("");
    let showAddForm = $state(false);
    let newTitle = $state("");
    let newUrl = $state("");

    // Watch for changes to initialSites
    $effect(() => {
        sites = [...initialSites];
    });

    function startEdit(index: number) {
        editingIndex = index;
        editTitle = sites[index].title;
        editUrl = sites[index].url;
    }

    function saveEdit() {
        if (editingIndex === null) return;
        
        if (!editTitle.trim() || !editUrl.trim()) {
            toast.error("Title and URL are required");
            return;
        }

        sites[editingIndex] = {
            title: editTitle.trim(),
            url: editUrl.trim()
        };
        
        editingIndex = null;
        editTitle = "";
        editUrl = "";
    }

    function cancelEdit() {
        editingIndex = null;
        editTitle = "";
        editUrl = "";
    }

    function removeSite(index: number) {
        sites = sites.filter((_, i) => i !== index);
        if (editingIndex === index) {
            cancelEdit();
        } else if (editingIndex !== null && editingIndex > index) {
            editingIndex--;
        }
    }

    function showAddSiteForm() {
        showAddForm = true;
        newTitle = "";
        newUrl = "";
    }

    function addSite() {
        if (!newTitle.trim() || !newUrl.trim()) {
            toast.error("Title and URL are required");
            return;
        }

        sites = [...sites, {
            title: newTitle.trim(),
            url: newUrl.trim()
        }];

        showAddForm = false;
        newTitle = "";
        newUrl = "";
    }

    function cancelAddSite() {
        showAddForm = false;
        newTitle = "";
        newUrl = "";
    }

    async function handleSave() {
        if (!collectionName.trim()) {
            toast.error("Collection name is required");
            return;
        }

        if (sites.length === 0) {
            toast.error("At least one site is required");
            return;
        }

        await onSave(collectionName.trim(), sites);
    }
</script>

{#if show}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black bg-opacity-50">
        <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-hidden flex flex-col">
            <!-- Header -->
            <div class="flex items-center justify-between p-4 border-b border-neutral-200 dark:border-neutral-700">
                <h3 class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
                    Create Collection
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
                <!-- Collection Name -->
                <div>
                    <label for="collection-name" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                        Collection Name
                    </label>
                    <input
                        type="text"
                        id="collection-name"
                        bind:value={collectionName}
                        oninput={(e) => onNameChange((e.target as HTMLInputElement).value)}
                        placeholder="Enter collection name..."
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    />
                </div>

                <!-- Sites List -->
                <div class="space-y-3">
                    <div class="flex items-center justify-between">
                        <h4 class="text-sm font-medium text-neutral-700 dark:text-neutral-300">
                            Sites ({sites.length})
                        </h4>
                        <button
                            onclick={showAddSiteForm}
                            class="flex items-center gap-1 px-3 py-1 text-sm text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-md transition-colors"
                        >
                            <Plus class="w-4 h-4" />
                            Add Site
                        </button>
                    </div>

                    <!-- Sites -->
                    {#each sites as site, index (index)}
                        <div class="bg-neutral-50 dark:bg-neutral-700 border border-neutral-200 dark:border-neutral-600 rounded-lg p-3">
                            {#if editingIndex === index}
                                <!-- Editing mode -->
                                <div class="space-y-2">
                                    <div>
                                        <label for="edit-title-{index}" class="block text-xs font-medium text-neutral-600 dark:text-neutral-400 mb-1">
                                            Title
                                        </label>
                                        <input
                                            type="text"
                                            id="edit-title-{index}"
                                            bind:value={editTitle}
                                            class="w-full px-2 py-1 text-sm border border-neutral-300 dark:border-neutral-500 rounded bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 focus:ring-1 focus:ring-blue-500"
                                        />
                                    </div>
                                    <div>
                                        <label for="edit-url-{index}" class="block text-xs font-medium text-neutral-600 dark:text-neutral-400 mb-1">
                                            URL
                                        </label>
                                        <input
                                            type="text"
                                            id="edit-url-{index}"
                                            bind:value={editUrl}
                                            class="w-full px-2 py-1 text-sm border border-neutral-300 dark:border-neutral-500 rounded bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 focus:ring-1 focus:ring-blue-500"
                                        />
                                    </div>
                                    <div class="flex justify-end gap-2">
                                        <button
                                            onclick={cancelEdit}
                                            class="p-1 text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300"
                                        >
                                            <X class="w-4 h-4" />
                                        </button>
                                        <button
                                            onclick={saveEdit}
                                            class="p-1 text-green-600 hover:text-green-700 dark:text-green-400"
                                        >
                                            <Check class="w-4 h-4" />
                                        </button>
                                    </div>
                                </div>
                            {:else}
                                <!-- Display mode -->
                                <div class="flex items-start justify-between">
                                    <div class="flex-1 min-w-0">
                                        <h5 class="text-sm font-medium text-neutral-900 dark:text-neutral-100 truncate">
                                            {site.title}
                                        </h5>
                                        <p class="text-xs text-neutral-500 dark:text-neutral-400 truncate">
                                            {site.url}
                                        </p>
                                    </div>
                                    <div class="flex items-center gap-1 ml-3">
                                        <button
                                            onclick={() => startEdit(index)}
                                            class="p-1 text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
                                            title="Edit site"
                                        >
                                            <PencilLine class="w-4 h-4" />
                                        </button>
                                        <button
                                            onclick={() => removeSite(index)}
                                            class="p-1 text-neutral-400 hover:text-red-600 dark:hover:text-red-400"
                                            title="Remove site"
                                        >
                                            <Trash2 class="w-4 h-4" />
                                        </button>
                                    </div>
                                </div>
                            {/if}
                        </div>
                    {/each}

                    <!-- Add Site Form -->
                    {#if showAddForm}
                        <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-700 rounded-lg p-3">
                            <div class="space-y-2">
                                <div>
                                    <label for="new-title" class="block text-xs font-medium text-neutral-600 dark:text-neutral-400 mb-1">
                                        Title
                                    </label>
                                    <input
                                        type="text"
                                        id="new-title"
                                        bind:value={newTitle}
                                        placeholder="Site title..."
                                        class="w-full px-2 py-1 text-sm border border-blue-300 dark:border-blue-600 rounded bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                                <div>
                                    <label for="new-url" class="block text-xs font-medium text-neutral-600 dark:text-neutral-400 mb-1">
                                        URL
                                    </label>
                                    <input
                                        type="text"
                                        id="new-url"
                                        bind:value={newUrl}
                                        placeholder="https://example.com"
                                        class="w-full px-2 py-1 text-sm border border-blue-300 dark:border-blue-600 rounded bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 focus:ring-1 focus:ring-blue-500"
                                    />
                                </div>
                                <div class="flex justify-end gap-2">
                                    <button
                                        onclick={cancelAddSite}
                                        class="px-3 py-1 text-xs text-neutral-600 dark:text-neutral-400 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded transition-colors"
                                    >
                                        Cancel
                                    </button>
                                    <button
                                        onclick={addSite}
                                        class="px-3 py-1 text-xs bg-blue-600 text-white hover:bg-blue-700 rounded transition-colors"
                                    >
                                        Add Site
                                    </button>
                                </div>
                            </div>
                        </div>
                    {/if}

                    {#if sites.length === 0}
                        <div class="text-center py-8 text-neutral-500 dark:text-neutral-400">
                            <p class="text-sm">No sites added yet</p>
                            <button
                                onclick={showAddSiteForm}
                                class="mt-2 text-sm text-blue-600 dark:text-blue-400 hover:underline"
                            >
                                Add your first site
                            </button>
                        </div>
                    {/if}
                </div>
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
                    onclick={handleSave}
                    disabled={!collectionName.trim() || sites.length === 0}
                    class="px-4 py-2 bg-blue-600 text-white hover:bg-blue-700 disabled:bg-blue-400 disabled:cursor-not-allowed rounded-md transition-colors"
                >
                    <Save class="w-4 h-4 inline mr-2" />
                    Save Collection
                </button>
            </div>
        </div>
    </div>
{/if}