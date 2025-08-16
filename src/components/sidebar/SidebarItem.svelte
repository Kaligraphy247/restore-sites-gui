<script lang="ts">
    import { createEventDispatcher } from "svelte";

    // Public props
    export interface Props {
        label: string;
        collapsed?: boolean;
        href?: string | null;
        target?: "_self" | "_blank" | "_parent" | "_top";
        rel?: string | null;
        active?: boolean;
        disabled?: boolean;
        badge?: string | null;
        ariaLabel?: string | null;
        className?: string;
        // Extra classes to apply only when active
        selectedClass?: string | null;
        icon?: import('svelte').Snippet;
        right?: import('svelte').Snippet;
    }

    let {
        label,
        collapsed = false,
        href = null,
        target = "_self",
        rel = null,
        active = false,
        disabled = false,
        badge = null,
        ariaLabel = null,
        className = "",
        selectedClass = null,
        icon,
        right
    }: Props = $props();

    const dispatch = createEventDispatcher<{ click: MouseEvent }>();


    function handleClick(e: MouseEvent) {
        if (disabled) {
            e.preventDefault();
            e.stopPropagation();
            return;
        }
        dispatch("click", e);
    }

    let elementTag: "a" | "button" = $derived(href && !disabled ? "a" : "button");

    let computedRel: string | null = $derived(
        rel ?? (target === "_blank" ? "noopener noreferrer" : null)
    );

    let classes: string = $derived(
        [
            // layout
            "flex w-full items-center gap-3 rounded-md outline-none transition",
            collapsed ? "justify-center px-2 py-2" : "justify-start px-3 py-2",
            // colors and interaction
            disabled
                ? "cursor-not-allowed opacity-50"
                : active
                  ? "bg-neutral-100 text-neutral-900 dark:bg-neutral-800 dark:text-neutral-50"
                  : "text-neutral-700 hover:bg-neutral-100 hover:text-neutral-900 dark:text-neutral-300 dark:hover:bg-neutral-800 dark:hover:text-neutral-50",
            // focus ring
            "focus-visible:ring-2 ring-offset-2 ring-blue-500 dark:ring-blue-400 ring-offset-white dark:ring-offset-neutral-900",
            // external className
            className,
            // selected extra class
            active && selectedClass ? selectedClass : "",
        ]
            .filter(Boolean)
            .join(" ")
    );
</script>

<!-- Wrapper provides group/focus context for tooltip -->
<div class="group relative" data-collapsed={collapsed} data-active={active}>
    <svelte:element
        this={elementTag}
        class={classes}
        aria-current={active ? "page" : undefined}
        aria-disabled={disabled ? "true" : undefined}
        aria-label={ariaLabel ?? (collapsed ? label : undefined)}
        href={elementTag === "a" ? href : undefined}
        target={elementTag === "a" ? target : undefined}
        rel={elementTag === "a" ? computedRel : undefined}
        role={elementTag === "a" ? undefined : "button"}
        type={elementTag === "button" ? "button" : undefined}
        onclick={handleClick}
    >
        <!-- Icon slot (always visible) -->
        <span class="inline-flex h-5 w-5 shrink-0 items-center justify-center">
            {@render icon?.()}
        </span>

        <!-- Label and right content (hidden when collapsed) -->
        {#if !collapsed}
            <span class="min-w-0 flex-1 truncate">{label}</span>
            <div class="ml-auto inline-flex items-center gap-2">
                {#if badge}
                    <span
                        class="inline-flex items-center rounded-md bg-neutral-100 px-1.5 py-0.5 text-[10px] font-semibold text-neutral-700 dark:bg-neutral-800 dark:text-neutral-200"
                    >
                        {badge}
                    </span>
                {/if}
                {@render right?.()}
            </div>
        {/if}
    </svelte:element>

    <!-- Tooltip shown only when collapsed -->
    {#if collapsed}
        <div
            class="pointer-events-none absolute left-full top-1/2 z-50 ml-2 -translate-y-1/2 whitespace-nowrap rounded-md bg-neutral-900 px-2 py-1 text-xs font-medium text-white opacity-0 shadow-lg ring-1 ring-black/10 transition-opacity duration-150 group-hover:opacity-100 group-focus-within:opacity-100 dark:bg-neutral-800"
            role="tooltip"
        >
            {label}
        </div>
    {/if}
</div>
