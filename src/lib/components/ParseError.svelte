<script lang="ts">
  import { fade } from 'svelte/transition';
  import { add_col, remove_col } from '$lib/stores/columns';
  import { parse_error } from '$lib/stores/parse_error';
  import { get } from 'svelte/store';

  parse_error.subscribe(async (_) => {
      setTimeout(() => {
        parse_error.update((_) => null)
      }, 3000);
  })

</script>


<div class="container overflow-y-scroll overflow-x-hidden text-center flex flex-col justify-center items-center h-full">
    <p class="text-md text-red-500 font-mono" out:fade> {$parse_error} </p>
</div>
