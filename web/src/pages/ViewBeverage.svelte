<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { stock } from "../modules/beverage";
  import BeverageDetails from "../lib/BeverageDetails.svelte";
  import IconAdd from "~icons/mdi/plus";
  import BeverageButton from "../lib/BeverageButton.svelte";

  const params = useParams();

  let beverage = $stock.find((b) => b.id === $params.id);
  let softdrinks = $stock.filter((s) => s.metadata.category === "softdrink");

  let shownCapabilities = beverage.capabilities.filter((c) =>
    ["unit", "shot"].includes(c)
  );
</script>

<section class="beverage-details">
  <div class="text-2xl mb-8">
    <BeverageDetails {beverage} />
  </div>
  <div class="capabilities grid gap-3">
    {#each shownCapabilities as capability}
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
  {#if beverage.capabilities.includes("mix")}
    <div class="mixWith mt-3 grid gap-3">
      {#each softdrinks as softdrink}
        <button>
          <BeverageButton beverage={softdrink} />
        </button>
      {/each}
    </div>
  {/if}
</section>
