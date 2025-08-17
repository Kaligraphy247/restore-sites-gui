<script lang="ts">
    import type { PageData } from "./$types";
    import {
        CircleArrowLeft,
        ArchiveRestore,
        Trash2,
        Plus,
        Check,
        X,
        Globe,
        Calendar,
        Settings,
        PencilLine
    } from "@lucide/svelte";
    import { goto } from "$app/navigation";
    import {
        updateCollection,
        deleteCollection,
        restoreCollection,
    } from "$lib/api/collections";
    import type { CollectionConfig, SiteEntry, Browser, BrowserMode } from "$lib/types/models";
    import { toast } from "svelte-sonner";
    import Dialog from "$lib/../components/Dialog.svelte";

    let { data }: { data: PageData } = $props();

    let collection = $state(data.collection);
    let editingUrl = $state<number | null>(null);
    let editUrlValue = $state("");
    let editTitleValue = $state("");
    let showAddForm = $state(false);
    let newUrl = $state("");
    let newTitle = $state("");

    // Dialog states
    let showNameDialog = $state(false);
    let showConfigDialog = $state(false);
    let tempName = $state("");
    let tempBrowser = $state<Browser>("Chrome");
    let tempMode = $state<BrowserMode>("Incognito");

    // Copy sites for editing - initialize empty first
    let sites = $state<SiteEntry[]>([]);

    // Selection state for sites
    let selectedSites = $state<Set<number>>(new Set());

    // Initialize state from collection data
    $effect(() => {
        if (collection) {
            sites = [...collection.sites];
            tempBrowser = collection.config?.browser || "Chrome";
            tempMode = collection.config?.mode || "Incognito";
        }
    });

    // Computed values for selection
    let selectedCount = $derived(selectedSites.size);
    let allSelected = $derived(
        selectedSites.size === sites.length && sites.length > 0,
    );
    let someSelected = $derived(
        selectedSites.size > 0 && selectedSites.size < sites.length,
    );

    function openNameDialog() {
        tempName = collection.name;
        showNameDialog = true;
    }

    async function handleSaveName() {
        if (!tempName.trim()) {
            toast.error("Name cannot be empty");
            return;
        }

        try {
            const updated = await updateCollection(
                collection.id,
                sites,
                tempName.trim(),
            );
            collection = updated;
            showNameDialog = false;
            toast.success("Collection name updated");
        } catch (error) {
            console.error("Failed to update name:", error);
            toast.error("Failed to update name");
        }
    }

    function openConfigDialog() {
        tempBrowser = collection.config?.browser || "Chrome";
        tempMode = collection.config?.mode || "Incognito";
        showConfigDialog = true;
    }

    async function handleSaveConfig() {
        try {
            // Create updated collection data with new config
            const updatedData = {
                sites,
                name: collection.name,
                config: {
                    browser: tempBrowser,
                    mode: tempMode,
                    custom_path: collection.config?.custom_path
                },
                created_at: collection.created_at
            };

            const updated = await updateCollection(collection.id, sites, collection.name, {
                browser: tempBrowser,
                mode: tempMode,
                custom_path: collection.config?.custom_path
            });

            collection = updated;
            showConfigDialog = false;
            toast.success("Collection settings updated");
        } catch (error) {
            console.error("Failed to update config:", error);
            toast.error("Failed to update settings");
        }
    }

    async function handleSaveEdit(index: number) {
        if (!editTitleValue.trim() || !editUrlValue.trim()) {
            toast.error("Title and URL cannot be empty");
            return;
        }

        sites[index] = {
            title: editTitleValue.trim(),
            url: editUrlValue.trim(),
        };

        try {
            const updated = await updateCollection(
                collection.id,
                sites,
                collection.name,
            );
            collection = updated;
            editingUrl = null;
            toast.success("Site updated");
        } catch (error) {
            console.error("Failed to update site:", error);
            toast.error("Failed to update site");
        }
    }

    async function handleRemoveSite(index: number) {
        if (sites.length === 1) {
            toast.error("Cannot remove the last site from a collection");
            return;
        }

        sites.splice(index, 1);

        // Update selected sites to account for removed item
        const newSelected = new Set<number>();
        selectedSites.forEach((selectedIndex) => {
            if (selectedIndex < index) {
                newSelected.add(selectedIndex);
            } else if (selectedIndex > index) {
                newSelected.add(selectedIndex - 1);
            }
            // Skip the removed index
        });
        selectedSites = newSelected;

        try {
            const updated = await updateCollection(
                collection.id,
                sites,
                collection.name,
            );
            collection = updated;
            toast.success("Site removed");
        } catch (error) {
            console.error("Failed to remove site:", error);
            toast.error("Failed to remove site");
        }
    }

    async function handleAddSite() {
        if (!newTitle.trim() || !newUrl.trim()) {
            toast.error("Title and URL cannot be empty");
            return;
        }

        const newSite: SiteEntry = {
            title: newTitle.trim(),
            url: newUrl.trim(),
        };

        sites.push(newSite);

        try {
            const updated = await updateCollection(
                collection.id,
                sites,
                collection.name,
            );
            collection = updated;
            showAddForm = false;
            newTitle = "";
            newUrl = "";
            toast.success("Site added");
        } catch (error) {
            console.error("Failed to add site:", error);
            toast.error("Failed to add site");
        }
    }

    async function handleDeleteCollection() {
        if (
            !confirm(
                "Are you sure you want to delete this collection? This action cannot be undone.",
            )
        ) {
            return;
        }

        try {
            await deleteCollection(collection.id);
            toast.success("Collection deleted");
            goto("/collections");
        } catch (error) {
            console.error("Failed to delete collection:", error);
            toast.error("Failed to delete collection");
        }
    }

    async function handleRestore() {
        try {
            await restoreCollection(collection.sites);
            toast.success("Collection restored");
        } catch (error) {
            console.error("Failed to restore collection:", error);
            toast.error("Failed to restore collection");
        }
    }

    async function handleRestoreSelected() {
        if (selectedSites.size === 0) {
            toast.error("No sites selected");
            return;
        }

        try {
            const selectedSiteEntries = sites.filter((_, index) =>
                selectedSites.has(index),
            );
            await restoreCollection(selectedSiteEntries);
            toast.success(`${selectedSites.size} sites restored`);
        } catch (error) {
            console.error("Failed to restore selected sites:", error);
            toast.error("Failed to restore selected sites");
        }
    }

    function toggleSiteSelection(index: number) {
        const newSelected = new Set(selectedSites);
        if (newSelected.has(index)) {
            newSelected.delete(index);
        } else {
            newSelected.add(index);
        }
        selectedSites = newSelected;
    }

    function selectAll() {
        selectedSites = new Set(sites.map((_, index) => index));
    }

    function selectNone() {
        selectedSites = new Set();
    }

    function startEditUrl(index: number) {
        const site = sites[index];
        editingUrl = index;
        editTitleValue = site.title;
        editUrlValue = site.url;
    }

    function cancelEditUrl() {
        editingUrl = null;
        editTitleValue = "";
        editUrlValue = "";
    }


    function cancelAddSite() {
        showAddForm = false;
        newTitle = "";
        newUrl = "";
    }
</script>

<div class="space-y-6 mb-8">
    <!-- Header -->
    <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
            <a
                href="/collections"
                class="flex items-center gap-2 px-3 py-2 text-sm bg-neutral-50 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-600 transition-colors duration-200"
            >
                <CircleArrowLeft size={16} />
                Back
            </a>

            <div class="flex-1 min-w-0">
                <div>
                    <h1
                        class="text-2xl font-semibold text-neutral-800 dark:text-neutral-100 cursor-pointer hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                        ondblclick={openNameDialog}
                        title="Double-click to edit collection name"
                    >
                        {collection.name}
                    </h1>
                </div>
                <div
                    class="flex items-center gap-4 mt-1 text-sm text-neutral-500 dark:text-neutral-400"
                >
                    <div class="flex items-center gap-1">
                        <Calendar size={14} />
                        {new Date(collection.created_at).toLocaleDateString()}
                    </div>
                    <div class="flex items-center gap-1">
                        <Globe size={14} />
                        {sites.length} site{sites.length !== 1 ? "s" : ""}
                    </div>
                </div>
            </div>
        </div>

        <div class="flex items-center gap-2">
            <button
                onclick={selectedCount > 0
                    ? handleRestoreSelected
                    : handleRestore}
                class="flex items-center gap-2 px-4 py-2 text-sm bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors duration-200"
            >
                <ArchiveRestore size={16} />
                {#if selectedCount > 0}
                    Restore {selectedCount} Selected
                {:else}
                    Restore All
                {/if}
            </button>
            <button
                onclick={openConfigDialog}
                class="flex items-center gap-2 px-4 py-2 text-sm bg-neutral-50 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-600 transition-colors duration-200"
                title="Collection Settings"
            >
                <Settings size={16} />
                Config
            </button>
            <button
                onclick={handleDeleteCollection}
                class="flex items-center gap-2 px-4 py-2 text-sm bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-md hover:bg-red-100 dark:hover:bg-red-900/80 transition-colors duration-200"
            >
                <Trash2 size={16} />
                Delete
            </button>
        </div>
    </div>

    <!-- Selection Controls -->
    {#if sites.length > 0}
        <div
            class="flex items-center justify-between bg-neutral-50 dark:bg-neutral-800/50 rounded-lg p-3"
        >
            <div class="flex items-center gap-4">
                <label class="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={allSelected}
                        indeterminate={someSelected}
                        onchange={(e) => {
                          if ((e.target as HTMLInputElement).checked) {
                                selectAll();
                            } else {
                                selectNone();
                            }
                        }}
                        class="w-4 h-4 text-blue-600 bg-white dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 rounded focus:ring-blue-500 dark:focus:ring-blue-600 focus:ring-2"
                    />
                    <span
                        class="text-sm font-medium text-neutral-700 dark:text-neutral-300"
                    >
                        {#if allSelected}
                            All sites selected
                        {:else if someSelected}
                            {selectedCount} of {sites.length} sites selected
                        {:else}
                            Select all sites
                        {/if}
                    </span>
                </label>
            </div>
            <div class="flex items-center gap-2">
                <button
                    onclick={selectAll}
                    class="px-3 py-1 text-xs bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors"
                    disabled={allSelected}
                >
                    Select All
                </button>
                <button
                    onclick={selectNone}
                    class="px-3 py-1 text-xs bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors"
                    disabled={selectedCount === 0}
                >
                    Clear
                </button>
            </div>
        </div>
    {/if}

    <!-- Sites List -->
    <div class="space-y-3 h-[400px] overflow-y-auto pr-4 pb-8 -mr-2">
        {#each sites as site, index}
            <div
                class="group bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-sm hover:shadow-md transition-all duration-200"
            >
                <div class="p-4">
                    {#if editingUrl === index}
                        <!-- Editing mode -->
                        <div class="space-y-3">
                            <div>
                                <!-- svelte-ignore a11y_label_has_associated_control -->
                                <label
                                    class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                                >
                                    Title
                                </label>
                                <input
                                    bind:value={editTitleValue}
                                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                    placeholder="Site title"
                                />
                            </div>
                            <div>
                                <!-- svelte-ignore a11y_label_has_associated_control -->
                                <label
                                    class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                                >
                                    URL
                                </label>
                                <input
                                    bind:value={editUrlValue}
                                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                    placeholder="https://example.com"
                                />
                            </div>
                            <div class="flex items-center gap-2">
                                <button
                                    onclick={() => handleSaveEdit(index)}
                                    class="flex items-center gap-1 px-3 py-1.5 text-sm bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 rounded-md hover:bg-green-100 dark:hover:bg-green-900/80 transition-colors"
                                >
                                    <Check size={14} />
                                    Save
                                </button>
                                <button
                                    onclick={cancelEditUrl}
                                    class="flex items-center gap-1 px-3 py-1.5 text-sm bg-neutral-50 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-600 transition-colors"
                                >
                                    <X size={14} />
                                    Cancel
                                </button>
                            </div>
                        </div>
                    {:else}
                        <!-- Display mode -->
                        <div class="flex items-center gap-3">
                            <label class="flex items-center cursor-pointer">
                                <input
                                    type="checkbox"
                                    checked={selectedSites.has(index)}
                                    onchange={() => toggleSiteSelection(index)}
                                    class="w-4 h-4 text-blue-600 bg-white dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 rounded focus:ring-blue-500 dark:focus:ring-blue-600 focus:ring-2"
                                />
                            </label>
                            <!-- <div
                                class="w-2 h-2 bg-neutral-400 dark:bg-neutral-500 rounded-full flex-shrink-0"
                            ></div> -->
                            <div class="flex-1 min-w-0">
                                <div
                                    class="font-medium text-neutral-900 dark:text-neutral-100 truncate"
                                    title={site.title}
                                >
                                    {site.title}
                                </div>
                                <div
                                    class="text-sm text-neutral-500 dark:text-neutral-400 truncate"
                                    title={site.url}
                                >
                                    {site.url}
                                </div>
                            </div>
                            <div
                                class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
                            >
                                <button
                                    onclick={() => startEditUrl(index)}
                                    class="p-2 text-neutral-500 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                                    title="Edit site"
                                >
                                    <PencilLine size={16} />
                                </button>
                                <button
                                    onclick={() => handleRemoveSite(index)}
                                    class="p-2 text-neutral-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded transition-colors"
                                    title="Remove site"
                                >
                                    <Trash2 size={16} />
                                </button>
                            </div>
                        </div>
                    {/if}
                </div>
            </div>
        {/each}

        <!-- Add Site Form -->
        {#if showAddForm}
            <div
                class="bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-sm"
            >
                <div class="p-4 space-y-3">
                    <h3
                        class="font-medium text-neutral-900 dark:text-neutral-100"
                    >
                        Add New Site
                    </h3>
                    <div>
                        <!-- svelte-ignore a11y_label_has_associated_control -->
                        <label
                            class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                        >
                            Title
                        </label>
                        <input
                            bind:value={newTitle}
                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            placeholder="Site title"
                            onkeydown={(e) => {
                                if (e.key === "Enter") handleAddSite();
                                if (e.key === "Escape") cancelAddSite();
                            }}
                        />
                    </div>
                    <div>
                        <!-- svelte-ignore a11y_label_has_associated_control -->
                        <label
                            class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
                        >
                            URL
                        </label>
                        <input
                            bind:value={newUrl}
                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            placeholder="https://example.com"
                            onkeydown={(e) => {
                                if (e.key === "Enter") handleAddSite();
                                if (e.key === "Escape") cancelAddSite();
                            }}
                        />
                    </div>
                    <div class="flex items-center gap-2">
                        <button
                            onclick={handleAddSite}
                            class="flex items-center gap-1 px-3 py-1.5 text-sm bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors"
                        >
                            <Plus size={14} />
                            Add Site
                        </button>
                        <button
                            onclick={cancelAddSite}
                            class="flex items-center gap-1 px-3 py-1.5 text-sm bg-neutral-50 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-600 transition-colors"
                        >
                            <X size={14} />
                            Cancel
                        </button>
                    </div>
                </div>
            </div>
        {:else}
            <!-- Add Site Button -->
            <button
                onclick={() => (showAddForm = true)}
                class="w-full p-4 bg-white dark:bg-neutral-800 border border-dashed border-neutral-300 dark:border-neutral-600 rounded-lg hover:border-blue-500 dark:hover:border-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/10 transition-all duration-200 mb-12"
            >
                <div
                    class="flex items-center justify-center gap-2 text-neutral-500 dark:text-neutral-400 hover:text-blue-600 dark:hover:text-blue-400"
                >
                    <Plus size={20} />
                    <span class="font-medium">Add New Site</span>
                </div>
            </button>
        {/if}
    </div>
</div>

<!-- Name Edit Dialog -->
<Dialog
    open={showNameDialog}
    title="Edit Collection Name"
    description="Change the name of this collection"
    onclose={() => showNameDialog = false}
>
    <div class="space-y-4">
        <div>
            <label for="collection-name" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">
                Collection Name
            </label>
            <input
                id="collection-name"
                bind:value={tempName}
                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="Enter collection name"
                onkeydown={(e) => {
                    if (e.key === 'Enter') handleSaveName();
                    if (e.key === 'Escape') showNameDialog = false;
                }}
            />
        </div>

        <div class="flex items-center gap-2 pt-4">
            <button
                onclick={handleSaveName}
                disabled={!tempName.trim()}
                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-md disabled:cursor-not-allowed transition-colors"
            >
                Save Changes
            </button>
            <button
                onclick={() => showNameDialog = false}
                class="px-4 py-2 bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors"
            >
                Cancel
            </button>
        </div>
    </div>
</Dialog>

<!-- Config Dialog -->
<Dialog
    open={showConfigDialog}
    title="Collection Settings"
    description="Configure default browser and mode for this collection"
    onclose={() => showConfigDialog = false}
    maxWidth="lg"
>
    <div class="space-y-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
                <label for="browser-select" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">
                    Default Browser
                </label>
                <select
                    id="browser-select"
                    bind:value={tempBrowser}
                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                    <option value="Chrome">Chrome</option>
                    <option value="Firefox">Firefox</option>
                    <option value="Safari">Safari</option>
                    <option value="Edge">Edge</option>
                </select>
                <p class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">
                    Browser to open links in when restoring this collection
                </p>
            </div>

            <div>
                <label for="mode-select" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">
                    Browsing Mode
                </label>
                <select
                    id="mode-select"
                    bind:value={tempMode}
                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                    <option value="Normal">Normal</option>
                    <option value="Incognito">Incognito</option>
                    <option value="Private">Private</option>
                </select>
                <p class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">
                    {#if tempMode === "Normal"}
                        Open in regular browser windows
                    {:else if tempMode === "Incognito"}
                        Open in incognito/private windows (Chrome/Edge)
                    {:else}
                        Open in private windows (Firefox/Safari)
                    {/if}
                </p>
            </div>
        </div>

        <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
            <div class="flex items-start gap-3">
                <div class="w-5 h-5 bg-blue-100 dark:bg-blue-900/40 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                    <Settings size={12} class="text-blue-600 dark:text-blue-400" />
                </div>
                <div>
                    <p class="text-sm text-blue-800 dark:text-blue-200 font-medium">
                        Collection-specific settings
                    </p>
                    <p class="text-xs text-blue-700 dark:text-blue-300 mt-1">
                        These settings will be used when restoring this collection. You can override them temporarily using the restore options.
                    </p>
                </div>
            </div>
        </div>

        <div class="flex items-center gap-2 pt-4">
            <button
                onclick={handleSaveConfig}
                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors"
            >
                Save Settings
            </button>
            <button
                onclick={() => showConfigDialog = false}
                class="px-4 py-2 bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors"
            >
                Cancel
            </button>
        </div>
    </div>
</Dialog>
