<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";


  let items = $state<string[]>([]);
  let queue: string[] = [];


  // Drip-feed items from queue to items with a tiny delay
  async function process_queue() {
    while (true) {
      if (queue.length > 0) {
        const next = queue.shift();
        if (next && !items.includes(next)) {
          items = [...items, next];
        }
      }
      await new Promise((r) => setTimeout(r, 40)); // 25 items/sec
    }
  }

  listen("clear", (_) => {
    items = [];
    queue = [];
  });
  
  listen("query-chunk", (event) => {
    const newChunk = event.payload as string[];
    console.log("Recieved: ", newChunk.length);
    queue = [...queue, ...newChunk];
  });

  onMount(async () => {
    await process_queue()
  })
</script>

<div class="container overflow-y-auto overflow-x-hidden col-span-2">

  
  {#if items.length === 0}
    <p class="text-gray-400">Loading results...</p>
  {/if}

  <ul class="flex flex-col items-center justify-center">
    {#each items as item (item)}
      <li
        in:fly={{ y: 20, duration: 120 }}
        class="my-1 p-2 hover:border rounded-md"
      >
        {item}
      </li>
    {/each}
  </ul>
</div>



<style>
  .container {
  direction: rtl;          /* Flip the layout direction */
  overflow-y: scroll;      /* Scrollbar visible */
  scrollbar-gutter: stable;
}

.container > * {
  direction: ltr;          /* Restore text direction for children */
}
</style>