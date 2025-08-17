<script lang="ts">
    import { Plus, X } from "@lucide/svelte";
    import type { SiteEntry } from "$lib/types/models";
    import SiteCard from "./SiteCard.svelte";

    interface Props {
        sites: SiteEntry[];
        selectedSites: Set<number>;
        editingUrl: number | null;
        editTitleValue: string;
        editUrlValue: string;
        showAddForm: boolean;
        newTitle: string;
        newUrl: string;
        onToggleSiteSelection: (index: number) => void;
        onStartEditUrl: (index: number) => void;
        onSaveEdit: (index: number) => void;
        onCancelEdit: () => void;
        onRemoveSite: (index: number) => void;
        onEditTitleChange: (value: string) => void;
        onEditUrlChange: (value: string) => void;
        onShowAddForm: (show: boolean) => void;
        onNewTitleChange: (value: string) => void;
        onNewUrlChange: (value: string) => void;
        onAddSite: () => void;
        onCancelAddSite: () => void;
    }

    let {
        sites,
        selectedSites,
        editingUrl,
        editTitleValue,
        editUrlValue,
        showAddForm,
        newTitle,
        newUrl,
        onToggleSiteSelection,
        onStartEditUrl,
        onSaveEdit,
        onCancelEdit,
        onRemoveSite,
        onEditTitleChange,
        onEditUrlChange,
        onShowAddForm,
        onNewTitleChange,
        onNewUrlChange,
        onAddSite,
        onCancelAddSite
    }: Props = $props();
</script>

<div class="space-y-3 h-[400px] overflow-y-auto pr-4 pb-8 -mr-2">
    {#each sites as site, index}
        <SiteCard
            {site}
            {index}
            isEditing={editingUrl === index}
            isSelected={selectedSites.has(index)}
            editTitle={editTitleValue}
            editUrl={editUrlValue}
            onToggleSelection={onToggleSiteSelection}
            onStartEdit={onStartEditUrl}
            onSaveEdit={onSaveEdit}
            onCancelEdit={onCancelEdit}
            onRemove={onRemoveSite}
            onEditTitleChange={onEditTitleChange}
            onEditUrlChange={onEditUrlChange}
        />
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
                        value={newTitle}
                        oninput={(e) => onNewTitleChange((e.target as HTMLInputElement).value)}
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="Site title"
                        onkeydown={(e) => {
                            if (e.key === "Enter") onAddSite();
                            if (e.key === "Escape") onCancelAddSite();
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
                        value={newUrl}
                        oninput={(e) => onNewUrlChange((e.target as HTMLInputElement).value)}
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="https://example.com"
                        onkeydown={(e) => {
                            if (e.key === "Enter") onAddSite();
                            if (e.key === "Escape") onCancelAddSite();
                        }}
                    />
                </div>
                <div class="flex items-center gap-2">
                    <button
                        onclick={onAddSite}
                        class="flex items-center gap-1 px-3 py-1.5 text-sm bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded-md hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors"
                    >
                        <Plus size={14} />
                        Add Site
                    </button>
                    <button
                        onclick={onCancelAddSite}
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
            onclick={() => onShowAddForm(true)}
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