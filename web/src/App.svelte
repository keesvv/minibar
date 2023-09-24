<script lang="ts">
  import { onMount } from "svelte";
  import type { Beverage } from "./modules/beverage";
  import { api } from "./modules/api";
  import BeverageTile from "./lib/BeverageTile.svelte";

  let beverages: Beverage[] = [];

  onMount(async () => {
    beverages = await api("beverages").json();
  });
</script>

<section class="catalog grid grid-flow-col gap-3 p-10">
  {#each beverages as beverage}
    <BeverageTile {beverage} />
  {/each}
</section>
