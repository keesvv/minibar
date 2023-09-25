<script lang="ts">
  import { stock } from "../modules/beverage";
  import BeverageButton from "../lib/BeverageButton.svelte";
  import { Link } from "svelte-navigator";
</script>

<section class="catalog grid md:grid-flow-col gap-3">
  {#each $stock.filter((b) => b.capabilities.length) as beverage}
    <Link to="/details/{beverage.id}">
      <BeverageButton {beverage} />
    </Link>
  {/each}
</section>
