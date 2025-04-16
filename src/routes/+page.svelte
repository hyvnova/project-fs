<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import user_config from "../lib/stores/user_config";
  import { listen } from "@tauri-apps/api/event";

  let input: string = $state("");
  let items: string[] = $state([]);

  async function update_file_list() {
    items = (await invoke("read_dir", {
      initial_path: null,
      path: input,
      limit: 21,
      showFullPath: false,
    })) as string[];
  }

  async function parse_query() {
    await invoke("stream_query", {
      q: input,
      limit: 100, // only want 500 files at most
      chunkSize: 10
    });
  }

  listen("query-chunk", (event) => {
    const newChunk = event.payload as string[];
    console.log("Recieved: ", newChunk.length);
    items = [...items, ...newChunk];
    items = new Array(...(new Set(items)))
  });

  listen("clear", (_) => {
    items = [];
  });

  onMount(async () => {
    await parse_query();
  });
</script>

<main
  class=" bg-black h-screen max-h-screen flex flex-col items-center justify-center text-white overflow-auto
  p-2"
>
  <input
    type="text"
    placeholder="Search"
    class="border border-white rounded-sm p-3 m-2 min-w-lg min-h-14"
    oninput={parse_query}
    bind:value={input}
  />

  {#if items.length === 0}
    <p class="text-gray-400">Loading results...</p>
  {/if}

  <div class="container overflow-y-scroll max-h-screen">
    <ul
      class="flex flex-col items-center justify-center max-h-screen overflow-y-scroll"
    >
      {#each items as item}
        <li>
          {item}
        </li>
      {/each}
    </ul>
  </div>
</main>
