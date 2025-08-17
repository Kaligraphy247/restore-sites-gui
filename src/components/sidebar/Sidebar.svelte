<script lang="ts">
    import { ChevronRight } from "@lucide/svelte";
    import { onMount } from "svelte";

    interface Props {
        defaultCollapsed?: boolean;
        widthExpanded?: number;
        widthCollapsed?: number;
        persistKey?: string | null;
        ariaLabel?: string;
        brand?: import('svelte').Snippet<[{ collapsed: boolean; isAnimating: boolean }]>;
        children?: import('svelte').Snippet<[{ collapsed: boolean }]>;
        footer?: import('svelte').Snippet<[{ collapsed: boolean }]>;
        // Callback props to replace createEventDispatcher
        ontoggle?: (collapsed: boolean) => void;
    }

    let {
        defaultCollapsed = false,
        widthExpanded = 180,
        widthCollapsed = 64,
        persistKey = "sidebar:collapsed",
        ariaLabel = "Sidebar",
        brand,
        children,
        footer,
        ontoggle,
    } = $props();

    // Component state
    let collapsed = $state(defaultCollapsed);
    let isAnimating = $state(false);
    let animationTimeoutId: number | undefined;

    onMount(() => {
        if (typeof window !== "undefined" && persistKey) {
            const stored = localStorage.getItem(persistKey);
            if (stored !== null) {
                collapsed = stored === "1";
            }
        }
    });

    function toggle() {
        isAnimating = true;
        collapsed = !collapsed;

        // Clear any existing timeout
        if (animationTimeoutId) {
            clearTimeout(animationTimeoutId);
        }

        // Reset animation state after transition duration (300ms)
        animationTimeoutId = setTimeout(() => {
            isAnimating = false;
        }, 300);

        if (persistKey) {
            try {
                localStorage.setItem(persistKey, collapsed ? "1" : "0");
            } catch {
                // ignore storage errors
            }
        }
        ontoggle?.(collapsed);
    }

    let currentWidth = $derived(collapsed ? widthCollapsed : widthExpanded);
</script>

<aside
    class="relative flex h-screen flex-col border-r bg-white text-neutral-900 transition-all duration-300 ease-in-out dark:border-neutral-800 dark:bg-neutral-900 dark:text-neutral-50 overflow-hidden shadow-lg"
    style={`width:${currentWidth}px;min-width:${currentWidth}px;`}
    data-collapsed={collapsed}
    aria-label={ariaLabel}
>
    <!-- Header / Brand -->
    <div
        class="flex h-14 items-center justify-between gap-2 border-b px-3 dark:border-neutral-800"
    >
        <div class="flex min-w-0 items-center gap-2 overflow-hidden">
            {@render brand?.({ collapsed, isAnimating })}
        </div>

        <button
            type="button"
            class="inline-flex h-8 w-8 items-center justify-center rounded-md border border-neutral-200 bg-white text-neutral-600 hover:bg-neutral-50 hover:text-neutral-900 active:bg-neutral-100 dark:border-neutral-800 dark:bg-neutral-900 dark:text-neutral-300 dark:hover:bg-neutral-800 dark:hover:text-neutral-50 transition-all duration-200"
            aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
            aria-expanded={!collapsed}
            onclick={toggle}
        >
            <div class="transition-transform duration-300 ease-in-out {collapsed ? 'rotate-0' : 'rotate-180'}">
                <ChevronRight size={18} />
            </div>
        </button>
    </div>

    <!-- Main navigation area -->
    <nav class="flex-1 overflow-y-auto px-2 py-2 scroll-smooth">
        <div class="space-y-1 transition-all duration-300 ease-in-out">
            <!-- Expose collapsed as a slot prop so consumers can adapt labels/icons -->
            {@render children?.({ collapsed })}
        </div>
    </nav>

    <!-- Footer / actions -->
    <div class="border-t px-2 py-2 dark:border-neutral-800 transition-all duration-300 ease-in-out">
        {@render footer?.({ collapsed })}
    </div>
</aside>
