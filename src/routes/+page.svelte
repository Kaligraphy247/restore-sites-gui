<script lang="ts">
  // import { invoke } from "@tauri-apps/api/tauri";

  // let name = "";
  // let greetMsg = "";

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   greetMsg = await invoke("greet", { name });
  // }
  type Site = {
    alias: string;
    incognito: boolean;
    open: boolean;
    url: string;
  };

  type SitesList = Array<Site>;

  let sites: SitesList = [];
  sites.push({
    url: "https://github.com",
    incognito: true,
    open: true,
    alias: "Github",
  });
  sites.push({
    url: "https://google.com",
    incognito: true,
    open: true,
    alias: "",
  });
  $: console.log(sites);

  let siteUrl: string = "";
  $: console.log(siteUrl);

  function addSite() {
    if (siteUrl === "") return;
    sites = [
      ...sites,
      { url: siteUrl, incognito: true, open: true, alias: "" },
    ];
    // siteUrl = "";
    console.log("sites", sites);
  }
</script>

<section class="py-4">
  <div class="text-center">
    <h1 class="text-4xl text-center font-semibold">Restore Sites GUI</h1>
    <p class="text-lg my-1">
      Restore previously open website to Incognito mode (Default)
    </p>
  </div>

  <div class="flex items-center justify-center space-x-2">
    <input
      type="text"
      placeholder="URL"
      class="text-black h-10 w-3/5 rounded pl-1"
      bind:value={siteUrl}
    />
    <button
      on:click={addSite}
      class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
      >Add</button
    >
  </div>

  <p class="text-lg my-1 mx-10 font-semibold">Total {sites.length}</p>
  {#if sites.length === 0}
    <div
      class="flex items-center justify-center h-[400px] max-h-[400px] border mx-10 border-dashed rounded"
    >
      <h3 class="text-xl italic font-medium">No sites yet</h3>
    </div>
  {:else}
    <ul
      class="border mx-10 p-2 h-[400px] max-h-[400px] overflow-y-auto space-y-4"
    >
      {#each sites as site, index}
        <li class="p-2 border rounded-lg text-lg">
          {index + 1}.
          {site.alias ? site.alias : site.url}
          <!-- {site.url} -->
          <input type="checkbox" bind:checked={site.incognito} disabled />
          <input type="checkbox" bind:checked={site.open} disabled />
          <button
            class="bg-red-500 hover:bg-red-600 text-white font-bold py-1 px-4 rounded"
            >Delete</button
          >
        </li>
      {/each}
    </ul>
  {/if}
  <div class="mx-10 mt-4">
    <button
      class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded w-full"
      >Restore</button
    >
  </div>
</section>
