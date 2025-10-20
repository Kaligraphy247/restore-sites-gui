<script lang="ts">
  import {
    Settings,
    CircleCheck,
    Globe,
    CircleX,
    Plus,
    PencilLine,
    Trash2,
    RefreshCw,
    Download,
    Upload,
  } from "@lucide/svelte";
  import type { PageData } from "./$types";
  import type { BrowserProfile, BrowserMode, Browser } from "$lib/types/models";
  import {
    getBrowserProfiles,
    createBrowserProfile,
    updateBrowserProfile,
    deleteBrowserProfile,
    checkBrowserDetection,
    setDefaultBrowserMode,
  } from "$lib/api/profiles";
  import { exportDatabaseToFile } from "$lib/api/collections";
  import { toast } from "svelte-sonner";

  let { data }: { data: PageData } = $props();

  let profiles = $state(data.profiles);
  let defaultMode = $state(data.defaultMode);
  let isLoading = $state(false);

  // Profile creation/editing state
  let showProfileDialog = $state(false);
  let editingProfile: BrowserProfile | null = $state(null);
  let profileForm = $state({
    id: "",
    name: "",
    browser: "Chrome" as Browser,
    mode: "Normal" as BrowserMode,
    custom_path: "",
    is_default: false,
  });

  // Reset form when dialog opens/closes
  function resetProfileForm() {
    profileForm = {
      id: "",
      name: "",
      browser: "Chrome",
      mode: "Normal",
      custom_path: "",
      is_default: false,
    };
    editingProfile = null;
  }

  function openCreateDialog() {
    resetProfileForm();
    showProfileDialog = true;
  }

  function openEditDialog(profile: BrowserProfile) {
    editingProfile = profile;
    profileForm = {
      id: profile.id,
      name: profile.name,
      browser: profile.browser,
      mode: profile.mode,
      custom_path: profile.custom_path || "",
      is_default: profile.is_default,
    };
    showProfileDialog = true;
  }

  async function saveProfile() {
    if (!profileForm.name.trim()) {
      toast.error("Profile name is required");
      return;
    }

    if (profileForm.name.length > 64) {
      toast.error("Profile name cannot exceed 64 characters");
      return;
    }

    isLoading = true;
    try {
      const profileData: BrowserProfile = {
        id: editingProfile ? editingProfile.id : generateProfileId(profileForm.name, profileForm.browser),
        name: profileForm.name.trim(),
        browser: profileForm.browser,
        mode: profileForm.mode,
        custom_path: profileForm.custom_path || undefined,
        is_default: profileForm.is_default,
        is_detected: false, // Will be updated by detection check
        created_at: editingProfile?.created_at || new Date().toISOString(),
        updated_at: new Date().toISOString(),
      };

      if (editingProfile) {
        await updateBrowserProfile(editingProfile.id, profileData);
        toast.success("Profile updated successfully");
      } else {
        await createBrowserProfile(profileData);
        toast.success("Profile created successfully");
      }

      // Refresh profiles list
      profiles = await getBrowserProfiles();
      showProfileDialog = false;
      resetProfileForm();
    } catch (error) {
      toast.error(`Failed to save profile: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  async function deleteProfile(profile: BrowserProfile) {
    if (!confirm(`Are you sure you want to delete the profile "${profile.name}"?`)) {
      return;
    }

    isLoading = true;
    try {
      await deleteBrowserProfile(profile.id);
      profiles = await getBrowserProfiles();
      toast.success("Profile deleted successfully");
    } catch (error) {
      toast.error(`Failed to delete profile: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  async function updateDetectionStatus() {
    isLoading = true;
    try {
      profiles = await checkBrowserDetection();
      toast.success("Detection status updated");
    } catch (error) {
      toast.error(`Failed to update detection status: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  async function updateDefaultMode(newMode: BrowserMode) {
    isLoading = true;
    try {
      await setDefaultBrowserMode(newMode);
      defaultMode = newMode;
      toast.success("Default browser mode updated");
    } catch (error) {
      toast.error(`Failed to update default mode: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  async function handleExportDatabase() {
    isLoading = true;
    try {
      const filePath = await exportDatabaseToFile();
      toast.success(`Database exported successfully to: ${filePath}`);
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      if (errorMessage.includes("Export cancelled")) {
        toast.info("Export cancelled");
      } else {
        toast.error(`Failed to export database: ${errorMessage}`);
      }
    } finally {
      isLoading = false;
    }
  }

  // Import function disabled for now - will be implemented later
  // async function handleImportDatabase() { ... }

  function generateProfileId(name: string, browser: Browser): string {
    const browserName = typeof browser === "string" ? browser.toLowerCase() : "custom";
    const safeName = name.toLowerCase().replace(/[^a-z0-9]/g, "-");
    return `${safeName}-${browserName}`;
  }

  function getBrowserDisplayName(browser: Browser): string {
    if (typeof browser === "string") {
      return browser;
    }
    return `Custom (${browser.Custom})`;
  }

  function getModeDisplayName(mode: BrowserMode): string {
    switch (mode) {
      case "Normal":
        return "Normal";
      case "Incognito":
        return "Incognito";
      case "Private":
        return "Private";
      default:
        return mode;
    }
  }
</script>

<svelte:head>
  <title>Settings - Restore Sites</title>
</svelte:head>

<div class="bg-neutral-50 dark:bg-neutral-900">
  <!-- Header -->
  <!-- <div class="bg-white dark:bg-neutral-800 shadow-sm border-b border-neutral-200 dark:border-neutral-800"> -->
  <div class="border-b border-b-neutral-800">
    <div class="max-w-4xl mx-auto px-6 py-4">
      <div class="flex items-center gap-3">
        <Settings class="w-6 h-6 text-neutral-600 dark:text-neutral-300" />
        <h1 class="text-2xl font-semibold text-neutral-900 dark:text-neutral-100">
          Settings
        </h1>
      </div>
    </div>
  </div>

  <!-- Main Content -->
  <div class="max-w-4xl mx-auto px-6 py-8 pb-16">
    <!-- Browser Profiles Section -->
    <section class="mb-12">
      <div class="flex items-center justify-between mb-4">
        <div>
          <h2 class="text-xl font-semibold text-neutral-900 dark:text-neutral-100 mb-2">
            Browser Profiles
          </h2>
          <p class="text-neutral-600 dark:text-neutral-400 text-xs">
            <!-- Manage reusable browser configurations for your collections -->
            Manage reusable browser configurations for your collections
          </p>
        </div>
        <div class="flex items-center gap-3">
          <button
            onclick={updateDetectionStatus}
            disabled={isLoading}
            class="flex items-center gap-2 px-3 py-2 text-sm bg-neutral-100 dark:bg-neutral-700 text-neutral-700 dark:text-neutral-300 rounded-md hover:bg-neutral-200 dark:hover:bg-neutral-600 transition-colors duration-200 disabled:opacity-50"
            title="Update Detection Status"
          >
            <RefreshCw class="w-4 h-4" />
            Refresh
          </button>
          <button
            onclick={openCreateDialog}
            class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 whitespace-nowrap flex-shrink-0"
          >
            <Plus class="w-4 h-4" />
            <span>Create Profile</span>
          </button>
        </div>
      </div>

      <!-- Profiles List -->
      <div class="space-y-3">
        {#each profiles as profile (profile.id)}
          <div class="bg-white dark:bg-neutral-800 rounded-lg border border-neutral-200 dark:border-neutral-700 p-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-4">
                <div class="flex items-center gap-2">
                  {#if profile.is_detected}
                    <CircleCheck class="w-5 h-5 text-green-500" />
                  {:else}
                    <CircleX class="w-5 h-5 text-red-500" />
                  {/if}
                  <div>
                    <div class="flex items-center gap-2">
                      <span class="font-medium text-neutral-900 dark:text-neutral-100">
                        {profile.name}
                      </span>
                      {#if profile.is_default}
                        <span class="px-2 py-1 text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full">
                          Default
                        </span>
                      {/if}
                    </div>
                    <div class="text-sm text-neutral-600 dark:text-neutral-400">
                      {getBrowserDisplayName(profile.browser)} • {getModeDisplayName(profile.mode)}
                      {#if profile.custom_path}
                        • Custom Path
                      {/if}
                    </div>
                  </div>
                </div>
              </div>
              <div class="flex items-center gap-2">
                <button
                  onclick={() => openEditDialog(profile)}
                  class="p-2 text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-neutral-100 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded-md transition-colors duration-200"
                  title="Edit Profile"
                >
                  <PencilLine class="w-4 h-4" />
                </button>
                <button
                  onclick={() => deleteProfile(profile)}
                  class="p-2 text-neutral-600 dark:text-neutral-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded-md transition-colors duration-200"
                  title="Delete Profile"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
        {/each}

        {#if profiles.length === 0}
          <div class="text-center py-12 bg-white dark:bg-neutral-800 rounded-lg border border-neutral-200 dark:border-neutral-700">
            <Globe class="w-12 h-12 text-neutral-400 mx-auto mb-4" />
            <h3 class="text-lg font-medium text-neutral-900 dark:text-neutral-100 mb-2">
              No Browser Profiles
            </h3>
            <p class="text-neutral-600 dark:text-neutral-400 mb-4">
              Create your first browser profile to get started
            </p>
            <button
              onclick={openCreateDialog}
              class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200"
            >
              Create Profile
            </button>
          </div>
        {/if}
      </div>
    </section>

    <!-- Global Settings Section -->
    <section>
      <div class="mb-6">
        <h2 class="text-xl font-semibold text-neutral-900 dark:text-neutral-100 mb-2">
          Global Settings
        </h2>
        <p class="text-neutral-600 dark:text-neutral-400">
          System-wide default settings used as fallbacks
        </p>
      </div>

      <div class="bg-white dark:bg-neutral-800 rounded-lg border border-neutral-200 dark:border-neutral-700 p-6">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="font-medium text-neutral-900 dark:text-neutral-100 mb-1">
              Default Browser Mode
            </h3>
            <p class="text-sm text-neutral-600 dark:text-neutral-400">
              Used when no profile is specified for a collection
            </p>
          </div>
          <select
            bind:value={defaultMode}
            onchange={(e) => updateDefaultMode((e.target as HTMLSelectElement).value as BrowserMode)}
            disabled={isLoading}
            class="px-3 py-2 bg-neutral-50 dark:bg-neutral-700 border border-neutral-300 dark:border-neutral-600 rounded-md text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:opacity-50"
          >
            <option value="Normal">Normal</option>
            <option value="Incognito">Incognito</option>
            <option value="Private">Private</option>
          </select>
        </div>
      </div>
    </section>

    <!-- Backup & Restore Section -->
    <section class="mt-12">
      <div class="mb-6">
        <h2 class="text-xl font-semibold text-neutral-900 dark:text-neutral-100 mb-2">
          Backup & Restore
        </h2>
        <p class="text-neutral-600 dark:text-neutral-400">
          Export and import your collections and settings
        </p>
      </div>

      <div class="bg-white dark:bg-neutral-800 rounded-lg border border-neutral-200 dark:border-neutral-700 p-6">
        <div class="space-y-4">
          <!-- Export Database -->
          <div class="flex items-center justify-between">
            <div>
              <h3 class="font-medium text-neutral-900 dark:text-neutral-100 mb-1">
                Export Database
              </h3>
              <p class="text-sm text-neutral-600 dark:text-neutral-400">
                Download a complete backup of all your collections and settings
              </p>
            </div>
            <button
              onclick={handleExportDatabase}
              disabled={isLoading}
              class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 disabled:opacity-50 whitespace-nowrap"
            >
              <Download class="w-4 h-4" />
              Export
            </button>
          </div>

          <!-- Divider -->
          <div class="border-t border-neutral-200 dark:border-neutral-700"></div>

          <!-- Import Database -->
          <div class="flex items-center justify-between">
            <div>
              <h3 class="font-medium text-neutral-900 dark:text-neutral-100 mb-1">
                Import Database
              </h3>
              <p class="text-sm text-neutral-600 dark:text-neutral-400">
                Restore from a backup file. Choose to merge or replace existing data
              </p>
            </div>
            <button
              disabled={true}
              class="flex items-center gap-2 px-4 py-2 bg-gray-400 text-gray-600 rounded-md cursor-not-allowed grayscale opacity-50 whitespace-nowrap"
              title="Import functionality temporarily disabled"
            >
              <Upload class="w-4 h-4" />
              Import
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>

<!-- Profile Create/Edit Dialog -->
{#if showProfileDialog}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-2 bg-black bg-opacity-50">
    <div class="bg-white dark:bg-neutral-800 rounded-lg shadow-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
      <div class="flex items-center justify-between p-4 border-b border-neutral-200 dark:border-neutral-700">
        <h3 class="text-lg font-semibold text-neutral-900 dark:text-neutral-100">
          {editingProfile ? "Edit Profile" : "Create Profile"}
        </h3>
        <button
          onclick={() => (showProfileDialog = false)}
          class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300"
          aria-label="Close dialog"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <form onsubmit={saveProfile} class="p-4 space-y-3">
        <!-- Profile Name -->
        <div>
          <label for="profile-name" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
            Profile Name
          </label>
          <input
            type="text"
            id="profile-name"
            bind:value={profileForm.name}
            maxlength="64"
            required
            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            placeholder="e.g., Default Chrome, Work Firefox"
          />
          <div class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">
            {profileForm.name.length}/64 characters
          </div>
        </div>

        <!-- Browser -->
        <div>
          <label for="profile-browser" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
            Browser
          </label>
          <select
            id="profile-browser"
            bind:value={profileForm.browser}
            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="Chrome">Chrome</option>
            <option value="Firefox">Firefox</option>
            <option value="Safari">Safari</option>
            <option value="Edge">Edge</option>
          </select>
        </div>

        <!-- Browser Mode -->
        <div>
          <label for="profile-mode" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
            Browser Mode
          </label>
          <select
            id="profile-mode"
            bind:value={profileForm.mode}
            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="Normal">Normal</option>
            <option value="Incognito">Incognito</option>
            <option value="Private">Private</option>
          </select>
        </div>

        <!-- Custom Path -->
        <div>
          <label for="profile-path" class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-1">
            Custom Browser Path (Optional)
          </label>
          <input
            type="text"
            id="profile-path"
            bind:value={profileForm.custom_path}
            class="w-full px-3 py-2 border border-neutral-300 dark:border-neutral-600 rounded-md bg-white dark:bg-neutral-700 text-neutral-900 dark:text-neutral-100 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            placeholder="e.g., /Applications/Google Chrome Beta.app"
          />
        </div>

        <!-- Is Default -->
        <div class="flex items-center">
          <input
            type="checkbox"
            bind:checked={profileForm.is_default}
            id="is_default"
            class="w-4 h-4 text-blue-600 bg-white dark:bg-neutral-700 border-neutral-300 dark:border-neutral-600 rounded focus:ring-blue-500"
          />
          <label for="is_default" class="ml-2 text-sm text-neutral-700 dark:text-neutral-300">
            Set as default profile
          </label>
        </div>

        <!-- Form Actions -->
        <div class="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onclick={() => (showProfileDialog = false)}
            class="px-4 py-2 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-100 dark:hover:bg-neutral-700 rounded-md transition-colors duration-200"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isLoading}
            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 disabled:opacity-50"
          >
            {editingProfile ? "Update" : "Create"} Profile
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
