<script lang="ts">
  import { stock } from "../modules/beverage";
  import BeverageTile from "../lib/BeverageTile.svelte";
</script>

<section class="catalog grid md:grid-flow-col gap-3">
  {#each $stock as beverage}
    <BeverageTile {beverage} />
  {/each}
</section>
