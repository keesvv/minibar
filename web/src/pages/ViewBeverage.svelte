<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { stock } from "../modules/beverage";
  import BeverageDetails from "../lib/BeverageDetails.svelte";
  import MixButton from "../lib/MixButton.svelte";
  import CapabilityButton from "../lib/CapabilityButton.svelte";

  const params = useParams();

  $: beverage = $stock.find((b) => b.id === $params.id);
  $: softdrinks = $stock.filter((s) => s.metadata.category === "softdrink");

  $: shownCapabilities =
    beverage?.capabilities.filter((c) => ["unit", "shot"].includes(c)) || [];
</script>

<section class="beverage-details">
  <div class="text-2xl mb-8">
    {#if beverage}
      <BeverageDetails {beverage} />
    {/if}
  </div>
  <div class="capabilities grid gap-3">
    {#each shownCapabilities as capability}
      <CapabilityButton {beverage} {capability} />
    {/each}
  </div>
  {#if beverage?.capabilities.includes("mix")}
    <div class="mixWith mt-3 grid gap-3">
      {#each softdrinks as softdrink}
        <MixButton {beverage} {softdrink} />
      {/each}
    </div>
  {/if}
</section>
