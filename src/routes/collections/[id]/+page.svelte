<script lang="ts">
    import type { PageData, PageProps } from "../../$types";
    import { CircleArrowLeft } from "@lucide/svelte";
    const { params }: PageProps = $props();
    // console.log(params);
</script>

<a
    href="/collections"
    class="w-24 py-1 px-2 rounded-md border absolute right-4 flex"
>
    <span class="w-4 h-4 mr-4">
        <CircleArrowLeft />
    </span>
    Back
</a>
