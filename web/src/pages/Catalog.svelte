<script lang="ts">
  import { _ } from "svelte-i18n";
  import { Link } from "svelte-navigator";
  import { stock } from "../modules/beverage";
  import BeverageInfo from "../lib/BeverageInfo.svelte";
  import WaterButton from "../lib/WaterButton.svelte";
</script>

<section class="catalog grid md:grid-flow-col gap-3">
  <WaterButton />
  {#each $stock.filter((b) => b.capabilities.length) as beverage}
    <Link to="/details/{beverage.id}" class="btn">
      <BeverageInfo {beverage} />
    </Link>
  {/each}
</section>
