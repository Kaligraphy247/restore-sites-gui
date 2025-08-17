<script lang="ts">
    import { Check, X, PencilLine, Trash2 } from "@lucide/svelte";
    import type { SiteEntry } from "$lib/types/models";

    interface Props {
        site: SiteEntry;
        index: number;
        isEditing: boolean;
        isSelected: boolean;
        editTitle: string;
        editUrl: string;
        onToggleSelection: (index: number) => void;
        onStartEdit: (index: number) => void;
        onSaveEdit: (index: number) => void;
        onCancelEdit: () => void;
        onRemove: (index: number) => void;
        onEditTitleChange: (value: string) => void;
        onEditUrlChange: (value: string) => void;
    }

    let {
        site,
        index,
        isEditing,
        isSelected,
        editTitle,
        editUrl,
        onToggleSelection,
        onStartEdit,
        onSaveEdit,
        onCancelEdit,
        onRemove,
        onEditTitleChange,
        onEditUrlChange
    }: Props = $props();
</script>

<div
    class="group bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-sm hover:shadow-md transition-all duration-200"
>
    <div class="p-4">
        {#if isEditing}
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
                        value={editTitle}
                        oninput={(e) => onEditTitleChange((e.target as HTMLInputElement).value)}
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
                        value={editUrl}
                        oninput={(e) => onEditUrlChange((e.target as HTMLInputElement).value)}
                        class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="https://example.com"
                    />
                </div>
                <div class="flex items-center gap-2">
                    <button
                        onclick={() => onSaveEdit(index)}
                        class="flex items-center gap-1 px-3 py-1.5 text-sm bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400 rounded-md hover:bg-green-100 dark:hover:bg-green-900/80 transition-colors"
                    >
                        <Check size={14} />
                        Save
                    </button>
                    <button
                        onclick={onCancelEdit}
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
                        checked={isSelected}
                        onchange={() => onToggleSelection(index)}
                        class="w-4 h-4 text-blue-600 bg-white dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 rounded focus:ring-blue-500 dark:focus:ring-blue-600 focus:ring-2"
                    />
                </label>
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
                        onclick={() => onStartEdit(index)}
                        class="p-2 text-neutral-500 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                        title="Edit site"
                    >
                        <PencilLine size={16} />
                    </button>
                    <button
                        onclick={() => onRemove(index)}
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