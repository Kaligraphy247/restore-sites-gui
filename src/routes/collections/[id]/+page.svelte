<script lang="ts">
    import type { PageData } from './$types';
    import {
        CircleArrowLeft,
        ArchiveRestore,
        Edit3,
        Trash2,
        Plus,
        Check,
        X,
        Globe,
        Calendar,
        Save
    } from "@lucide/svelte";
    import { goto } from '$app/navigation';
    import { updateCollection, deleteCollection, restoreCollection } from '$lib/api/collections';
    import type { SiteEntry } from '$lib/types/models';
    import { toast } from 'svelte-sonner';

    let { data }: { data: PageData } = $props();
    
    let collection = $state(data.collection);
    let editingName = $state(false);
    let editingUrl = $state<number | null>(null);
    let newName = $state(collection.name);
    let editUrlValue = $state("");
    let editTitleValue = $state("");
    let showAddForm = $state(false);
    let newUrl = $state("");
    let newTitle = $state("");

    // Copy sites for editing
    let sites = $state([...collection.sites]);

    async function handleSaveName() {
        if (!newName.trim()) {
            toast.error('Name cannot be empty');
            return;
        }
        
        try {
            const updated = await updateCollection(collection.id, sites, newName.trim());
            collection = updated;
            editingName = false;
            toast.success('Collection name updated');
        } catch (error) {
            console.error('Failed to update name:', error);
            toast.error('Failed to update name');
        }
    }

    async function handleSaveEdit(index: number) {
        if (!editTitleValue.trim() || !editUrlValue.trim()) {
            toast.error('Title and URL cannot be empty');
            return;
        }

        sites[index] = {
            title: editTitleValue.trim(),
            url: editUrlValue.trim()
        };

        try {
            const updated = await updateCollection(collection.id, sites, collection.name);
            collection = updated;
            editingUrl = null;
            toast.success('Site updated');
        } catch (error) {
            console.error('Failed to update site:', error);
            toast.error('Failed to update site');
        }
    }

    async function handleRemoveSite(index: number) {
        if (sites.length === 1) {
            toast.error('Cannot remove the last site from a collection');
            return;
        }

        sites.splice(index, 1);

        try {
            const updated = await updateCollection(collection.id, sites, collection.name);
            collection = updated;
            toast.success('Site removed');
        } catch (error) {
            console.error('Failed to remove site:', error);
            toast.error('Failed to remove site');
        }
    }

    async function handleAddSite() {
        if (!newTitle.trim() || !newUrl.trim()) {
            toast.error('Title and URL cannot be empty');
            return;
        }

        const newSite: SiteEntry = {
            title: newTitle.trim(),
            url: newUrl.trim()
        };

        sites.push(newSite);

        try {
            const updated = await updateCollection(collection.id, sites, collection.name);
            collection = updated;
            showAddForm = false;
            newTitle = "";
            newUrl = "";
            toast.success('Site added');
        } catch (error) {
            console.error('Failed to add site:', error);
            toast.error('Failed to add site');
        }
    }

    async function handleDeleteCollection() {
        if (!confirm('Are you sure you want to delete this collection? This action cannot be undone.')) {
            return;
        }

        try {
            await deleteCollection(collection.id);
            toast.success('Collection deleted');
            goto('/collections');
        } catch (error) {
            console.error('Failed to delete collection:', error);
            toast.error('Failed to delete collection');
        }
    }

    async function handleRestore() {
        try {
            await restoreCollection(collection.sites);
            toast.success('Collection restored');
        } catch (error) {
            console.error('Failed to restore collection:', error);
            toast.error('Failed to restore collection');
        }
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

    function cancelEditName() {
        editingName = false;
        newName = collection.name;
    }

    function cancelAddSite() {
        showAddForm = false;
        newTitle = "";
        newUrl = "";
    }
</script>

<div class="space-y-6">
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
                {#if editingName}
                    <div class="flex items-center gap-2">
                        <input
                            bind:value={newName}
                            class="text-2xl font-semibold bg-transparent border-b-2 border-blue-500 focus:outline-none text-neutral-800 dark:text-neutral-100"
                            placeholder="Collection name"
                            onkeydown={(e) => {
                                if (e.key === 'Enter') handleSaveName();
                                if (e.key === 'Escape') cancelEditName();
                            }}
                        />
                        <button 
                            onclick={handleSaveName}
                            class="p-1 text-green-600 hover:bg-green-50 dark:hover:bg-green-900/20 rounded"
                        >
                            <Check size={16} />
                        </button>
                        <button 
                            onclick={cancelEditName}
                            class="p-1 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded"
                        >
                            <X size={16} />
                        </button>
                    </div>
                {:else}
                    <div class="flex items-center gap-2">
                        <h1 class="text-2xl font-semibold text-neutral-800 dark:text-neutral-100">
                            {collection.name}
                        </h1>
                        <button 
                            onclick={() => editingName = true}
                            class="p-1 text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded transition-colors"
                        >
                            <Edit3 size={16} />
                        </button>
                    </div>
                {/if}
                <div class="flex items-center gap-4 mt-1 text-sm text-neutral-500 dark:text-neutral-400">
                    <div class="flex items-center gap-1">
                        <Calendar size={14} />
                        {new Date(collection.created_at).toLocaleDateString()}
                    </div>
                    <div class="flex items-center gap-1">
                        <Globe size={14} />
                        {sites.length} site{sites.length !== 1 ? 's' : ''}
                    </div>
                </div>
            </div>
        </div>
        
        <div class="flex items-center gap-2">
            <button
                onclick={handleRestore}
                class="flex items-center gap-2 px-4 py-2 text-sm bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors duration-200"
            >
                <ArchiveRestore size={16} />
                Restore All
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

    <!-- Sites List -->
    <div class="space-y-3 h-[450px] overflow-y-auto pr-2 pb-8">
        {#each sites as site, index}
            <div class="group bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-sm hover:shadow-md transition-all duration-200">
                <div class="p-4">
                    {#if editingUrl === index}
                        <!-- Editing mode -->
                        <div class="space-y-3">
                            <div>
                                <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                                    Title
                                </label>
                                <input
                                    bind:value={editTitleValue}
                                    class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                    placeholder="Site title"
                                />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
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
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 bg-neutral-400 dark:bg-neutral-500 rounded-full flex-shrink-0"></div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium text-neutral-900 dark:text-neutral-100 truncate" title={site.title}>
                                    {site.title}
                                </div>
                                <div class="text-sm text-neutral-500 dark:text-neutral-400 truncate" title={site.url}>
                                    {site.url}
                                </div>
                            </div>
                            <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                                <button 
                                    onclick={() => startEditUrl(index)}
                                    class="p-2 text-neutral-500 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                                    title="Edit site"
                                >
                                    <Edit3 size={16} />
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
            <div class="bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-sm">
                <div class="p-4 space-y-3">
                    <h3 class="font-medium text-neutral-900 dark:text-neutral-100">Add New Site</h3>
                    <div>
                        <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                            Title
                        </label>
                        <input
                            bind:value={newTitle}
                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            placeholder="Site title"
                            onkeydown={(e) => {
                                if (e.key === 'Enter') handleAddSite();
                                if (e.key === 'Escape') cancelAddSite();
                            }}
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
                            URL
                        </label>
                        <input
                            bind:value={newUrl}
                            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            placeholder="https://example.com"
                            onkeydown={(e) => {
                                if (e.key === 'Enter') handleAddSite();
                                if (e.key === 'Escape') cancelAddSite();
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
                onclick={() => showAddForm = true}
                class="w-full p-4 bg-white dark:bg-neutral-800 border border-dashed border-neutral-300 dark:border-neutral-600 rounded-lg hover:border-blue-500 dark:hover:border-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/10 transition-all duration-200"
            >
                <div class="flex items-center justify-center gap-2 text-neutral-500 dark:text-neutral-400 hover:text-blue-600 dark:hover:text-blue-400">
                    <Plus size={20} />
                    <span class="font-medium">Add New Site</span>
                </div>
            </button>
        {/if}
    </div>
</div>
