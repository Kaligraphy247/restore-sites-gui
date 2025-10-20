<script lang="ts">
    import { Plus, type Icon } from "@lucide/svelte";
    import type { Component } from "svelte";

    interface Props {
        onClick: () => void;
        icon?: Component<any>;
        title?: string;
        ariaLabel?: string;
    }

    let {
        onClick,
        icon = Plus,
        title = "Create new collection",
        ariaLabel = "Create new collection"
    }: Props = $props();

    let IconComponent = $derived(icon);
</script>

<button
    onclick={onClick}
    class="fixed bottom-6 right-6 z-40 w-14 h-14 bg-blue-600 hover:bg-blue-700 text-white rounded-full shadow-lg hover:shadow-xl transition-all duration-200 flex items-center justify-center group focus:ring-4 focus:ring-blue-300 dark:focus:ring-blue-800"
    {title}
    aria-label={ariaLabel}
>
    <IconComponent class="w-6 h-6 group-hover:scale-110 transition-transform duration-200" />
</button>