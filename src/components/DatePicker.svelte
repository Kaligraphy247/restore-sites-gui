<script lang="ts">
    import { ChevronLeft, ChevronRight, Calendar } from "@lucide/svelte";
    import { slide } from "svelte/transition";
    import { clickOutside } from "$lib/utils";

    interface Props {
        value?: Date | null;
        placeholder?: string;
        disabled?: boolean;
        min?: Date | null;
        max?: Date | null;
        id?: string;
        label?: string;
        onchange?: (date: Date | null) => void;
    }

    let {
        value = null,
        placeholder = "Select date",
        disabled = false,
        min = null,
        max = null,
        id,
        label,
        onchange,
    }: Props = $props();

    let isOpen = $state(false);
    let inputRef: HTMLInputElement;
    let selectedDate = $state(value);

    // Navigation state for manual month/year changes
    let navMonth = $state<number | null>(null);
    let navYear = $state<number | null>(null);

    // Derived view month/year that uses navigation state if available, otherwise selected date
    let viewMonth = $derived(
        navMonth ?? selectedDate?.getMonth() ?? new Date().getMonth()
    );
    let viewYear = $derived(
        navYear ?? selectedDate?.getFullYear() ?? new Date().getFullYear()
    );

    // Sync external value changes
    $effect(() => {
        selectedDate = value;
        // Reset navigation when external value changes
        if (value) {
            navMonth = null;
            navYear = null;
        }
    });

    // Format date for display
    function formatDate(date: Date | null): string {
        if (!date) return "";
        return date.toLocaleDateString("en-US", {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    // Get days in month
    function getDaysInMonth(month: number, year: number): number {
        return new Date(year, month + 1, 0).getDate();
    }

    // Get first day of month (0 = Sunday)
    function getFirstDayOfMonth(month: number, year: number): number {
        return new Date(year, month, 1).getDay();
    }

    // Generate calendar days
    let calendarDays = $derived.by(() => {
        const daysInMonth = getDaysInMonth(viewMonth, viewYear);
        const firstDay = getFirstDayOfMonth(viewMonth, viewYear);
        const days: (number | null)[] = [];

        // Add empty cells for days before month starts
        for (let i = 0; i < firstDay; i++) {
            days.push(null);
        }

        // Add days of month
        for (let day = 1; day <= daysInMonth; day++) {
            days.push(day);
        }

        return days;
    });

    // Check if date is disabled
    function isDateDisabled(day: number): boolean {
        const date = new Date(viewYear, viewMonth, day);
        if (min && date < min) return true;
        if (max && date > max) return true;
        return false;
    }

    // Check if date is today
    function isToday(day: number): boolean {
        const today = new Date();
        return (
            today.getDate() === day &&
            today.getMonth() === viewMonth &&
            today.getFullYear() === viewYear
        );
    }

    // Check if date is selected
    function isSelected(day: number): boolean {
        if (!selectedDate) return false;
        return (
            selectedDate.getDate() === day &&
            selectedDate.getMonth() === viewMonth &&
            selectedDate.getFullYear() === viewYear
        );
    }

    // Navigate months
    function previousMonth() {
        if (viewMonth === 0) {
            navMonth = 11;
            navYear = (navYear ?? viewYear) - 1;
        } else {
            navMonth = viewMonth - 1;
            navYear = navYear ?? viewYear;
        }
    }

    function nextMonth() {
        if (viewMonth === 11) {
            navMonth = 0;
            navYear = (navYear ?? viewYear) + 1;
        } else {
            navMonth = viewMonth + 1;
            navYear = navYear ?? viewYear;
        }
    }

    // Select date
    function selectDate(day: number) {
        if (isDateDisabled(day)) return;

        const newDate = new Date(viewYear, viewMonth, day);
        selectedDate = newDate;
        onchange?.(newDate);
        // Reset navigation to show selected date's month
        navMonth = null;
        navYear = null;
        isOpen = false;
    }

    // Clear selection
    function clearDate() {
        selectedDate = null;
        onchange?.(null);
        isOpen = false;
    }

    // Month names
    const monthNames = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Weekday names
    const weekdays = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    // Handle input click
    function handleInputClick() {
        if (!disabled) {
            isOpen = !isOpen;
        }
    }

    // Handle keyboard navigation
    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Escape") {
            isOpen = false;
            inputRef?.focus();
        }
    }

    // Close on outside click
    function handleClickOutside() {
        isOpen = false;
    }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="relative" use:clickOutside={handleClickOutside}>
    <!-- Label -->
    {#if label}
        <label
            for={id}
            class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1"
        >
            {label}
        </label>
    {/if}

    <!-- Input -->
    <div class="relative">
        <input
            {id}
            bind:this={inputRef}
            type="text"
            readonly
            value={formatDate(selectedDate)}
            {placeholder}
            {disabled}
            onclick={handleInputClick}
            class="w-full pl-10 pr-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-800 text-neutral-900 dark:text-neutral-100 placeholder-neutral-500 dark:placeholder-neutral-400 focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
            aria-haspopup="dialog"
        />
        <div
            class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
        >
            <Calendar
                size={16}
                class="text-neutral-400 dark:text-neutral-500"
            />
        </div>
    </div>

    <!-- Calendar Dropdown -->
    {#if isOpen}
        <div
            class="absolute z-[50] mt-1 bg-white dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 rounded-lg shadow-xl p-4 min-w-[280px] max-w-[320px]"
            style="top: 100%; left: 0; right: auto;"
            transition:slide={{ duration: 200 }}
            role="dialog"
            aria-label="Calendar"
        >
            <!-- Header -->
            <div class="flex items-center justify-between mb-4">
                <button
                    onclick={previousMonth}
                    class="p-1 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded transition-colors"
                    aria-label="Previous month"
                >
                    <ChevronLeft size={16} />
                </button>

                <h3 class="font-medium text-neutral-900 dark:text-neutral-100">
                    {monthNames[viewMonth]}
                    {viewYear}
                </h3>

                <button
                    onclick={nextMonth}
                    class="p-1 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded transition-colors"
                    aria-label="Next month"
                >
                    <ChevronRight size={16} />
                </button>
            </div>

            <!-- Weekday headers -->
            <div class="grid grid-cols-7 gap-1 mb-2">
                {#each weekdays as day}
                    <div
                        class="text-center text-xs font-medium text-neutral-500 dark:text-neutral-400 p-1"
                    >
                        {day}
                    </div>
                {/each}
            </div>

            <!-- Calendar grid -->
            <div class="grid grid-cols-7 gap-1">
                {#each calendarDays as day}
                    {#if day === null}
                        <div class="p-2"></div>
                    {:else}
                        <button
                            onclick={() => selectDate(day)}
                            disabled={isDateDisabled(day)}
                            class="p-2 text-sm rounded transition-colors relative
                                {isSelected(day)
                                ? 'bg-blue-600 text-white'
                                : isToday(day)
                                  ? 'bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400'
                                  : 'hover:bg-neutral-100 dark:hover:bg-neutral-700 text-neutral-900 dark:text-neutral-100'}
                                disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-transparent"
                            aria-label="{monthNames[
                                viewMonth
                            ]} {day}, {viewYear}"
                        >
                            {day}
                        </button>
                    {/if}
                {/each}
            </div>

            <!-- Footer -->
            <div
                class="flex items-center justify-between mt-4 pt-3 border-t border-neutral-200 dark:border-neutral-700"
            >
                <button
                    onclick={clearDate}
                    class="text-sm text-neutral-500 dark:text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-200 transition-colors"
                >
                    Clear
                </button>

                <button
                    onclick={() => (isOpen = false)}
                    class="text-sm bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded transition-colors"
                >
                    Close
                </button>
            </div>
        </div>
    {/if}
</div>
