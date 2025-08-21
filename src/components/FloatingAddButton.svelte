<script lang="ts">
    import { Plus } from "@lucide/svelte";
    
    interface Props {
        onClick: () => void;
    }
    
    let { onClick }: Props = $props();
</script>

<button
    onclick={onClick}
    class="fixed bottom-6 right-6 z-40 w-14 h-14 bg-blue-600 hover:bg-blue-700 text-white rounded-full shadow-lg hover:shadow-xl transition-all duration-200 flex items-center justify-center group focus:ring-4 focus:ring-blue-300 dark:focus:ring-blue-800"
    title="Create new collection"
    aria-label="Create new collection"
>
    <Plus class="w-6 h-6 group-hover:scale-110 transition-transform duration-200" />
</button>