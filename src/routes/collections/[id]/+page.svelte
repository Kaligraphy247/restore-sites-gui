<script lang="ts">
    import type { PageData } from "./$types";
    import {
        CircleArrowLeft,
        ArchiveRestore,
        Trash2,
        Globe,
        Calendar,
        Settings,
        Wrench,
    } from "@lucide/svelte";
    import { goto } from "$app/navigation";
    import {
        updateCollection,
        deleteCollection,
        restoreCollection,
    } from "$lib/api/collections";
    import type {
        SiteEntry,
        Browser,
        BrowserMode,
        BrowserProfile,
    } from "$lib/types/models";
    import { getBrowserProfiles } from "$lib/api/profiles";
    import { toast } from "svelte-sonner";
    import Dialog from "$lib/../components/Dialog.svelte";
    import SelectionControls from "$lib/../components/SelectionControls.svelte";
    import SiteList from "$lib/../components/SiteList.svelte";

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
    let showDeleteDialog = $state(false);
    let tempName = $state("");
    let tempBrowser = $state<Browser>("Chrome");
    let tempMode = $state<BrowserMode>("Incognito");

    // Browser profiles state
    let profiles = $state<BrowserProfile[]>([]);
    let tempProfileId = $state<string>("");
    let useCustomConfig = $state(false);

    // Copy sites for editing - initialize empty first
    let sites = $state<SiteEntry[]>([]);

    // Selection state for sites
    let selectedSites = $state<Set<number>>(new Set());

    // Load profiles on mount
    $effect(() => {
        async function loadProfiles() {
            try {
                profiles = await getBrowserProfiles();
            } catch (error) {
                console.error("Failed to load profiles:", error);
                toast.error("Failed to load browser profiles");
            }
        }
        loadProfiles();
    });

    // Initialize state from collection data
    $effect(() => {
        if (collection) {
            sites = [...collection.sites];
            tempProfileId = collection.config?.browser_profile_id || "";
            tempBrowser = collection.config?.browser || "Chrome";
            tempMode = collection.config?.mode || "Incognito";
            useCustomConfig =
                !collection.config?.browser_profile_id &&
                !!(collection.config?.browser || collection.config?.mode);
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
        tempProfileId = collection.config?.browser_profile_id || "";
        tempBrowser = collection.config?.browser || "Chrome";
        tempMode = collection.config?.mode || "Incognito";
        useCustomConfig =
            !collection.config?.browser_profile_id &&
            !!(collection.config?.browser || collection.config?.mode);
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
                    custom_path: collection.config?.custom_path,
                },
                created_at: collection.created_at,
            };

            // Build config based on whether using profile or custom config
            const config = useCustomConfig
                ? {
                      browser_profile_id: undefined,
                      browser: tempBrowser,
                      mode: tempMode,
                      custom_path: collection.config?.custom_path,
                  }
                : {
                      browser_profile_id: tempProfileId || undefined,
                      browser: undefined,
                      mode: undefined,
                      custom_path: collection.config?.custom_path,
                  };

            const updated = await updateCollection(
                collection.id,
                sites,
                collection.name,
                config,
            );

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
                collection.config,
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
                collection.config,
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
                collection.config,
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

    function openDeleteDialog() {
        showDeleteDialog = true;
    }

    async function confirmDeleteCollection() {
        showDeleteDialog = false;
        
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
            await restoreCollection(collection.sites, collection.config);
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
            await restoreCollection(selectedSiteEntries, collection.config);
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

    // Component callback wrappers
    function handleEditTitleChange(value: string) {
        editTitleValue = value;
    }

    function handleEditUrlChange(value: string) {
        editUrlValue = value;
    }

    function handleNewTitleChange(value: string) {
        newTitle = value;
    }

    function handleNewUrlChange(value: string) {
        newUrl = value;
    }

    function handleShowAddForm(show: boolean) {
        showAddForm = show;
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
                        class="text-xl font-semibold text-neutral-800 dark:text-neutral-100 cursor-pointer hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
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
                class="flex items-center gap-2 px-4 py-2 text-sm bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors duration-200 whitespace-nowrap flex-shrink-0"
            >
                <ArchiveRestore size={16} />
                {#if selectedCount > 0}
                    Restore {selectedCount}
                {:else}
                    Restore All
                {/if}
            </button>
            <button
                onclick={openConfigDialog}
                class="flex items-center gap-2 px-4 py-2 text-sm bg-neutral-50 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-100 dark:hover:bg-neutral-600 transition-colors duration-200"
                title="Collection Settings"
            >
                <Wrench size={16} />
                Config
            </button>
            <button
                onclick={openDeleteDialog}
                class="flex items-center gap-2 px-4 py-2 text-sm bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-md hover:bg-red-100 dark:hover:bg-red-900/80 transition-colors duration-200"
            >
                <Trash2 size={16} />
                Delete
            </button>
        </div>
    </div>

    <!-- Selection Controls -->
    <SelectionControls
        sitesCount={sites.length}
        {selectedCount}
        {allSelected}
        {someSelected}
        onSelectAll={selectAll}
        onSelectNone={selectNone}
    />

    <!-- Sites List -->
    <SiteList
        {sites}
        {selectedSites}
        {editingUrl}
        {editTitleValue}
        {editUrlValue}
        {showAddForm}
        {newTitle}
        {newUrl}
        onToggleSiteSelection={toggleSiteSelection}
        onStartEditUrl={startEditUrl}
        onSaveEdit={handleSaveEdit}
        onCancelEdit={cancelEditUrl}
        onRemoveSite={handleRemoveSite}
        onEditTitleChange={handleEditTitleChange}
        onEditUrlChange={handleEditUrlChange}
        onShowAddForm={handleShowAddForm}
        onNewTitleChange={handleNewTitleChange}
        onNewUrlChange={handleNewUrlChange}
        onAddSite={handleAddSite}
        onCancelAddSite={cancelAddSite}
    />
</div>

<!-- Name Edit Dialog -->
<Dialog
    open={showNameDialog}
    title="Edit Collection Name"
    description="Change the name of this collection"
    onclose={() => (showNameDialog = false)}
>
    <div class="space-y-4">
        <div>
            <label
                for="collection-name"
                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2"
            >
                Collection Name
            </label>
            <input
                id="collection-name"
                bind:value={tempName}
                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                placeholder="Enter collection name"
                onkeydown={(e) => {
                    if (e.key === "Enter") handleSaveName();
                    if (e.key === "Escape") showNameDialog = false;
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
                onclick={() => (showNameDialog = false)}
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
    onclose={() => (showConfigDialog = false)}
    maxWidth="lg"
>
    <div class="space-y-6">
        <!-- Profile Selection -->
        <div>
            <label
                for="profile-select"
                class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2"
            >
                Browser Profile
            </label>
            <select
                id="profile-select"
                bind:value={tempProfileId}
                disabled={useCustomConfig}
                class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
                <option value=""
                    >No profile selected (use global defaults)</option
                >
                {#each profiles as profile (profile.id)}
                    <option value={profile.id}>
                        {profile.is_detected ? "🟢" : "🔴"}
                        {profile.name} ({profile.browser} • {profile.mode})
                    </option>
                {/each}
            </select>
            <div class="flex items-center justify-between mt-2">
                <p class="text-xs text-neutral-500 dark:text-neutral-400">
                    🟢 Detected, 🔴 Not detected
                </p>
                <button
                    onclick={() => goto("/settings")}
                    class="text-xs text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 underline"
                >
                    Manage Profiles
                </button>
            </div>
        </div>

        <!-- Custom Configuration Toggle -->
        <div class="flex items-center">
            <input
                type="checkbox"
                id="use-custom"
                bind:checked={useCustomConfig}
                class="w-4 h-4 text-blue-600 bg-white dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 rounded focus:ring-blue-500"
            />
            <label
                for="use-custom"
                class="ml-2 text-sm text-neutral-700 dark:text-neutral-300"
            >
                Use custom configuration (one-time)
            </label>
        </div>

        <!-- Custom Configuration Fields (shown when checkbox is checked) -->
        {#if useCustomConfig}
            <div
                class="grid grid-cols-1 md:grid-cols-2 gap-4 p-4 bg-neutral-50 dark:bg-neutral-800 rounded-lg border border-neutral-200 dark:border-neutral-700"
            >
                <div>
                    <label
                        for="custom-browser-select"
                        class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2"
                    >
                        Browser
                    </label>
                    <select
                        id="custom-browser-select"
                        bind:value={tempBrowser}
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    >
                        <option value="Chrome">Chrome</option>
                        <option value="Firefox">Firefox</option>
                        <option value="Safari">Safari</option>
                        <option value="Edge">Edge</option>
                    </select>
                </div>

                <div>
                    <label
                        for="custom-mode-select"
                        class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2"
                    >
                        Mode
                    </label>
                    <select
                        id="custom-mode-select"
                        bind:value={tempMode}
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    >
                        <option value="Normal">Normal</option>
                        <option value="Incognito">Incognito</option>
                        <option value="Private">Private</option>
                    </select>
                </div>
            </div>
        {/if}

        <!-- Info Box -->
        <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
            <div class="flex items-start gap-3">
                <div
                    class="w-5 h-5 bg-blue-100 dark:bg-blue-900/40 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5"
                >
                    <Settings
                        size={12}
                        class="text-blue-600 dark:text-blue-400"
                    />
                </div>
                <div>
                    <p
                        class="text-sm text-blue-800 dark:text-blue-200 font-medium"
                    >
                        Configuration Priority
                    </p>
                    <p class="text-xs text-blue-700 dark:text-blue-300 mt-1">
                        {#if useCustomConfig}
                            Using custom configuration for this collection only.
                        {:else if tempProfileId}
                            Using selected browser profile. If profile is
                            missing, will fall back to global defaults.
                        {:else}
                            Using global default settings. Create or select a
                            profile for consistent browser configuration.
                        {/if}
                    </p>
                </div>
            </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex items-center gap-2 pt-4">
            <button
                onclick={handleSaveConfig}
                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors"
            >
                Save Settings
            </button>
            <button
                onclick={() => (showConfigDialog = false)}
                class="px-4 py-2 bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors"
            >
                Cancel
            </button>
        </div>
    </div>
</Dialog>

<!-- Delete Confirmation Dialog -->
<Dialog
    open={showDeleteDialog}
    title="Delete Collection"
    description="This action cannot be undone. Are you sure you want to delete this collection?"
    onclose={() => (showDeleteDialog = false)}
>
    <div class="space-y-4">
        <div class="bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
            <div class="flex items-start gap-3">
                <div class="w-5 h-5 bg-red-100 dark:bg-red-900/40 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                    <Trash2 size={12} class="text-red-600 dark:text-red-400" />
                </div>
                <div>
                    <p class="text-sm text-red-800 dark:text-red-200 font-medium">
                        You are about to delete "{collection.name}"
                    </p>
                    <p class="text-xs text-red-700 dark:text-red-300 mt-1">
                        This collection contains {collection.sites.length} sites. Once deleted, this data cannot be recovered.
                    </p>
                </div>
            </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex items-center gap-2 pt-4">
            <button
                onclick={confirmDeleteCollection}
                class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-md transition-colors"
            >
                Delete Collection
            </button>
            <button
                onclick={() => (showDeleteDialog = false)}
                class="px-4 py-2 bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded-md hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors"
            >
                Cancel
            </button>
        </div>
    </div>
</Dialog>
