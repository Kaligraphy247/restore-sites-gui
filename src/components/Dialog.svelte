<script lang="ts">
    import { X } from "@lucide/svelte";

    interface Props {
        open: boolean;
        title: string;
        description?: string;
        maxWidth?: "sm" | "md" | "lg" | "xl";
        showCloseButton?: boolean;
        onclose?: () => void;
        onopen?: () => void;
    }

    let {
        open = false,
        title,
        description,
        maxWidth = "md",
        showCloseButton = true,
        onclose,
        onopen,
        children
    }: Props & { children?: any } = $props();

    function handleClose() {
        onclose?.();
    }

    function handleBackdropClick(event: MouseEvent) {
        if (event.target === event.currentTarget) {
            handleClose();
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape' && open) {
            handleClose();
        }
    }

    const maxWidthClasses = {
        sm: "max-w-sm",
        md: "max-w-md",
        lg: "max-w-lg",
        xl: "max-w-xl"
    };

    $effect(() => {
        if (open) {
            onopen?.();
            // Focus trap and body scroll lock would go here in a full implementation
            document.body.style.overflow = 'hidden';
        } else {
            document.body.style.overflow = '';
        }

        return () => {
            document.body.style.overflow = '';
        };
    });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
    <!-- Backdrop -->
    <div
        class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
        onclick={handleBackdropClick}
        onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleClose()}
        role="dialog"
        aria-modal="true"
        aria-labelledby="dialog-title"
        aria-describedby={description ? "dialog-description" : undefined}
        tabindex="-1"
    >
        <!-- Dialog -->
        <div
            class="bg-white dark:bg-neutral-800 rounded-lg shadow-xl w-full {maxWidthClasses[maxWidth]} max-h-[90vh] overflow-y-auto"
            role="document"
        >
            <!-- Header -->
            <div class="flex items-center justify-between p-6 border-b border-neutral-200 dark:border-neutral-700">
                <div>
                    <h2 id="dialog-title" class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
                        {title}
                    </h2>
                    {#if description}
                        <p id="dialog-description" class="mt-1 text-sm text-neutral-600 dark:text-neutral-400">
                            {description}
                        </p>
                    {/if}
                </div>
                {#if showCloseButton}
                    <button
                        onclick={handleClose}
                        class="p-1 text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded transition-colors"
                        aria-label="Close dialog"
                    >
                        <X size={20} />
                    </button>
                {/if}
            </div>

            <!-- Content -->
            <div class="p-6">
                {@render children?.()}
            </div>
        </div>
    </div>
{/if}
