<script lang="ts">
  import { stock } from "../modules/beverage";
  import BeverageButton from "../lib/BeverageButton.svelte";
</script>

<section class="catalog grid md:grid-flow-col gap-3">
  {#each $stock as beverage}
    <BeverageButton {beverage} />
  {/each}
</section>
