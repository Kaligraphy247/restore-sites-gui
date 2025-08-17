<script lang="ts">
  import "../app.css";
  import Sidebar from "../components/sidebar/Sidebar.svelte";
  import SidebarItem from "../components/sidebar/SidebarItem.svelte";
  import { Folder, Settings, House } from "@lucide/svelte";
  import { Toaster } from "svelte-sonner";
  import { page } from "$app/state";
  import { fade, fly } from "svelte/transition";
  import { navigating } from "$app/stores";
  let { children } = $props();
  
  // Track route changes for transitions
  let currentPath = $state(page.url.pathname);
  $effect(() => {
    currentPath = page.url.pathname;
  });
</script>

<!-- Toast -->
<Toaster />
<!-- Toast -->

<div class="flex h-screen w-screen bg-white text-neutral-900 dark:bg-neutral-900 dark:text-neutral-50">
  <Sidebar ariaLabel="Main navigation" ontoggle={(collapsed: boolean) => console.log('Sidebar toggled:', collapsed)}>
    {#snippet brand({ collapsed, isAnimating }: { collapsed: boolean; isAnimating: boolean })}
      <div class="relative w-full min-h-[24px]">
        <!-- Collapsed state - RS -->
        <div class="absolute inset-0 flex items-center justify-center transition-all duration-200 ease-in-out {collapsed ? (isAnimating ? 'opacity-0' : 'opacity-100 delay-100') : 'opacity-0'}">
          <div class="text-sm font-extrabold tracking-wide">RS</div>
        </div>

        <!-- Expanded state - Restore Sites -->
        <div class="transition-all duration-200 ease-in-out {collapsed ? 'opacity-0' : (isAnimating ? 'opacity-0' : 'opacity-100 delay-100')}">
          <div class="flex items-center gap-2 font-extrabold whitespace-nowrap">
            <span>Restore Sites</span>
          </div>
        </div>
      </div>
    {/snippet}

    {#snippet children({ collapsed }: { collapsed: boolean })}
      <div class="space-y-1">
        <SidebarItem label="House" {collapsed} active={page.url.pathname === "/"} ariaLabel="House" href={"/"}>
          {#snippet icon()}
            <House size={18} />
          {/snippet}
        </SidebarItem>

        <SidebarItem label="Projects" {collapsed} active={page.url.pathname === "/projects"} ariaLabel="Projects" href={"/projects"}>
          {#snippet icon()}
            <Folder size={18} />
          {/snippet}
        </SidebarItem>
      </div>
    {/snippet}

    {#snippet footer({ collapsed }: { collapsed: boolean })}
      <SidebarItem label="Settings" {collapsed} ariaLabel="Settings">
        {#snippet icon()}
          <Settings size={18} />
        {/snippet}
      </SidebarItem>
    {/snippet}
  </Sidebar>

  <div class="min-w-0 flex-1 overflow-hidden">
    <header class="h-14 border-b px-4 dark:border-neutral-800 flex items-center">
      <h1 class="text-lg font-semibold">Restore Sites</h1>
    </header>
    <main class="p-4">
      {#key currentPath}
        <div
          in:fade={{ duration: 300, delay: 150 }}
          out:fade={{ duration: 150 }}
          class="w-full"
        >
          {@render children()}
        </div>
      {/key}
    </main>
  </div>
</div>
