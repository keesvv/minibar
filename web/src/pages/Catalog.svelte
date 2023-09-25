<script lang="ts">
  import { stock } from "../modules/beverage";
  import BeverageButton from "../lib/BeverageButton.svelte";
  import { Link } from "svelte-navigator";
  import IconWater from "~icons/mdi/water";
  import { _ } from "svelte-i18n";
</script>

<section class="catalog grid md:grid-flow-col gap-3">
  <button class="btn-alt flex gap-2 items-center">
    <IconWater />
    <span>{$_("water")}</span>
  </button>
  {#each $stock.filter((b) => b.capabilities.length) as beverage}
    <Link to="/details/{beverage.id}">
      <BeverageButton {beverage} />
    </Link>
  {/each}
</section>
