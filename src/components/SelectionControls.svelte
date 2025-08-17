<script lang="ts">
    interface Props {
        sitesCount: number;
        selectedCount: number;
        allSelected: boolean;
        someSelected: boolean;
        onSelectAll: () => void;
        onSelectNone: () => void;
    }

    let {
        sitesCount,
        selectedCount,
        allSelected,
        someSelected,
        onSelectAll,
        onSelectNone
    }: Props = $props();
</script>

{#if sitesCount > 0}
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
                            onSelectAll();
                        } else {
                            onSelectNone();
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
                        {selectedCount} of {sitesCount} sites selected
                    {:else}
                        Select all sites
                    {/if}
                </span>
            </label>
        </div>
        <div class="flex items-center gap-2">
            <button
                onclick={onSelectAll}
                class="px-3 py-1 text-xs bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 rounded hover:bg-blue-100 dark:hover:bg-blue-900/80 transition-colors"
                disabled={allSelected}
            >
                Select All
            </button>
            <button
                onclick={onSelectNone}
                class="px-3 py-1 text-xs bg-neutral-100 dark:bg-neutral-700 text-neutral-600 dark:text-neutral-300 rounded hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors"
                disabled={selectedCount === 0}
            >
                Clear
            </button>
        </div>
    </div>
{/if}