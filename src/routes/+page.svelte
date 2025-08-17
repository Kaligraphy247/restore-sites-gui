<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { toast } from "svelte-sonner";
    import { formatUrl } from "$lib/utils";

    async function saveSite() {
        if (sites === "") return;
        let formatted = formatUrl(sites);
        console.log({ formatted });

        try {
            const result = await invoke("save_collection", {
                request: {
                    sites: formatted,
                    config: {
                        browser: "chrome",
                        mode: "incognito",
                        custom_path: "",
                    },
                },
            });
            console.log("Project saved:", result);
            toast.success("Saved");
            sites = "";
        } catch (error) {
            console.error("Save failed:", error);
            toast.error("Failed to save");
        }
    }

    // let sites: string = $state("");
    let sites: string = $state(`
GitHub - langflow-ai/langflow: Langflow is a powerful tool for building and deploying AI-powered agents and workflows. https://github.com/langflow-ai/langflow?tab=readme-ov-file
What is Langflow? | Langflow Documentation https://docs.langflow.org/about-langflow
Flowise - Build AI Agents, Visually https://flowiseai.com/
1. Intermediate https://docmost.local.kaligraphy.dev/s/general/p/1-intermediate-xeSwC6MRWu
State Management | Tauri https://v2.tauri.app/develop/state-management/
Install Tailwind CSS with SvelteKit - Tailwind CSS https://tailwindcss.com/docs/installation/framework-guides/sveltekit
Svelte 5 migration guide • Docs • Svelte https://svelte.dev/docs/svelte/v5-migration-guide#Other-breaking-changes
Tauri + SvelteKit + Typescript App http://localhost:1420/projects
Lucide https://lucide.dev/icons/?search=Hous
tauri app in dark mode flashes on navigation - Google Search https://www.google.com/search?sca_esv=ada4534496032d39&rlz=1C5CHFA_enDE1065DE1065&q=tauri+app+in+dark+mode+flashes+on+navigation&spell=1&sa=X&ved=2ahUKEwjBkpeSuJCPAxVG2AIHHW8kAJ8QBSgAegQIGBAB&biw=1280&bih=1294&dpr=1
With dark mode stored on the client it is impossible to have page navigation without white flash? : r/reactjs https://www.reddit.com/r/reactjs/comments/1exk4ty/with_dark_mode_stored_on_the_client_it_is/
Dark mode - Core concepts - Tailwind CSS https://tailwindcss.com/docs/dark-mode#supporting-system-preference-and-manual-selection
Shiki https://shiki.style/
`);
    $inspect(sites);
</script>

<h2 class="font-semibold text-lg">Recent projects</h2>
<div class="border rounded-md grid grid-cols-3 w-full h-[144px]"></div>

<h2 class="font-semibold text-lg mt-4 mb-0">Add new Project</h2>

<textarea
    bind:value={sites}
    name="projects"
    placeholder="Enter a list of urls to be saved"
    class="w-full border rounded h-[220px] px-2 py-1 resize-none"
>
</textarea>

<div class="flex justify-between mt-2">
    <button class="bg-red-500 px-2 py-1 rounded-md w-20">Cancel</button>
    <button class="bg-blue-500 px-2 py-1 rounded-md w-20" onclick={saveSite}
        >Save</button
    >
</div>
