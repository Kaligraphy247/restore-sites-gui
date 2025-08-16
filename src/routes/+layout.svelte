<script lang="ts">
  import "../app.css";
  import Sidebar from "../components/sidebar/Sidebar.svelte";
  import SidebarItem from "../components/sidebar/SidebarItem.svelte";
  import { Folder, Settings, House } from "@lucide/svelte";
  let { children } = $props();
</script>

<div class="flex h-screen w-screen bg-white text-neutral-900 dark:bg-neutral-900 dark:text-neutral-50">
  <Sidebar ariaLabel="Main navigation">
    {#snippet brand({ collapsed })}
      {#if collapsed}
        <div class="mx-auto text-sm font-extrabold tracking-wide">RS</div>
      {:else}
        <div class="flex items-center gap-2 font-extrabold">
          <span>Restore Sites</span>
        </div>
      {/if}
    {/snippet}

    {#snippet children({ collapsed })}
      <SidebarItem label="House" {collapsed} active ariaLabel="House">
        {#snippet icon()}
          <House size={18} />
        {/snippet}
      </SidebarItem>

      <SidebarItem label="Projects" {collapsed} ariaLabel="Projects">
        {#snippet icon()}
          <Folder size={18} />
        {/snippet}
      </SidebarItem>
    {/snippet}

    {#snippet footer({ collapsed })}
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
      {@render children()}
    </main>
  </div>
</div>
