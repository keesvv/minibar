<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { stock } from "../modules/beverage";
  import BeverageDetails from "../lib/BeverageDetails.svelte";
  import IconAdd from "~icons/mdi/plus";

  const params = useParams();

  let beverage = $stock.find((b) => b.id === $params.id);
</script>

<section class="beverage-details">
  <div class="text-2xl mb-8">
    <BeverageDetails {beverage} />
  </div>
  <div class="capabilities grid">
    {#each beverage.capabilities as capability}
      <button class="btn flex gap-2 items-center">
        <IconAdd />
        {#if capability === "unit" && beverage.metadata.packaging}
          {$_(`beverage.packaging.${beverage.metadata.packaging}`)}
        {:else}
          {$_(`beverage.capability.${capability}`)}
        {/if}
      </button>
    {/each}
  </div>
</section>
