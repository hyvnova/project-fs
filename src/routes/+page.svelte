<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { add_col, columns, remove_col } from "$lib/stores/columns";
  import { listen } from "@tauri-apps/api/event";
  import { parse_errors } from "$lib/stores/parse_erros_arg";
  import { get } from "svelte/store";

  let input = $state<string>("");


  async function parse_query() {
    await invoke("stream_query", {
      q: input,
      limit: 50,
      chunkSize: 10,
    });
  }

  listen("parse-error", (error) => {
    console.log("parse error: ", error);

    parse_errors.update(errs => {
      errs.push(error.payload as string);
      return errs;
    })

  });

  
  parse_errors.subscribe(() => {
    if (get(parse_errors).length > 1) {
      add_col("ParseError");
    } else {
      remove_col("ParseError")
    }
  })

  onMount(async () => {
    await parse_query();
  });

  add_col("SearchResult")
</script>

<main
  class="bg-black h-screen max-h-screen flex flex-col items-center justify-center text-white overflow-auto p-2"
>
  <input
    type="text"
    placeholder="Search"
    class="border border-white rounded-sm p-3 m-2 min-w-lg min-h-12"
    oninput={parse_query}
    bind:value={input}
  />

  <div
    class="grid max-h-screen overflow-y-hidden gap-1"
    style="grid-template-columns: repeat({Object.keys($columns).length}, 1fr);"
  >
    {#each Object.entries($columns) as entry}
      {@const Component = entry[1]}
      <Component  />
    {/each}
  </div>
</main>
