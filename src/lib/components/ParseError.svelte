<script lang="ts">
  import { fade } from 'svelte/transition';
  import { add_col, remove_col } from '$lib/stores/columns';
  import { parse_errors } from '$lib/stores/parse_erros_arg';
  import { get } from 'svelte/store';

</script>


<div class="container overflow-y-scroll overflow-x-hidden">
  <h1>Errors</h1>
  {#each $parse_errors as text}
    <p class="text-sm text-red-500" out:fade> {text} </p>
  {/each}
</div>
