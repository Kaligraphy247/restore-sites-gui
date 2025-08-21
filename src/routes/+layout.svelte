<script lang="ts">
  import "../app.css";
  import Sidebar from "../components/sidebar/Sidebar.svelte";
  import SidebarItem from "../components/sidebar/SidebarItem.svelte";
  import { Settings, House, Library } from "@lucide/svelte";
  import { Toaster } from "svelte-sonner";
  import { page } from "$app/state";
  import { fade, fly } from "svelte/transition";
  import { navigating } from "$app/stores";
  import FloatingAddButton from "../components/FloatingAddButton.svelte";
  import CreateCollectionModal from "../components/collections/CreateCollectionModal.svelte";
  import { saveCollection, loadCollections } from "$lib/api/collections";
  import type { SiteEntry } from "$lib/types/models";
  import { toast } from "svelte-sonner";
  let { children } = $props();

  // Track route changes for transitions
  let currentPath = $state(page.url.pathname);
  $effect(() => {
    currentPath = page.url.pathname;
  });

  // Floating add button state
  let showCreateModal = $state(false);

  function openCreateModal() {
    showCreateModal = true;
  }

  function closeCreateModal() {
    showCreateModal = false;
  }

  async function handleCreateCollection(name: string, sites: SiteEntry[]) {
    try {
      await saveCollection(sites, {
        browser: "Chrome",
        mode: "Incognito",
      }, name);
      toast.success(`Collection "${name}" created with ${sites.length} sites`);
      showCreateModal = false;
      
      // If we're on the collections page, we might want to refresh
      if (currentPath.includes('/collections')) {
        // The collections page should handle its own refresh
        // We could emit a custom event here if needed
      }
    } catch (error) {
      console.error("Failed to create collection:", error);
      toast.error("Failed to create collection");
    }
  }
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
        <SidebarItem label="Home" {collapsed} active={page.url.pathname === "/"} ariaLabel="Home" href={"/"}>
          {#snippet icon()}
            <House size={18} />
          {/snippet}
        </SidebarItem>

        <SidebarItem label="Collections" {collapsed} active={page.url.pathname.includes("/collections")} ariaLabel="Collections" href={"/collections"}>
          {#snippet icon()}
            <Library size={18} />
          {/snippet}
        </SidebarItem>
      </div>
    {/snippet}

    {#snippet footer({ collapsed }: { collapsed: boolean })}
      <SidebarItem label="Settings" {collapsed} active={page.url.pathname === "/settings"} ariaLabel="Settings" href={"/settings"}>
        {#snippet icon()}
          <Settings size={18} />
        {/snippet}
      </SidebarItem>
    {/snippet}
  </Sidebar>

  <div class="min-w-0 flex-1 flex flex-col overflow-hidden">
    <header class="h-14 border-b px-4 dark:border-neutral-800 flex items-center flex-shrink-0">
      <h1 class="text-lg font-semibold">Restore Sites</h1>
    </header>
    <main class="flex-1 overflow-y-auto p-4">
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
  
  <!-- Floating Add Button -->
  <FloatingAddButton onClick={openCreateModal} />
  
  <!-- Global Create Collection Modal -->
  <CreateCollectionModal
    show={showCreateModal}
    onSave={handleCreateCollection}
    onCancel={closeCreateModal}
  />
</div>
