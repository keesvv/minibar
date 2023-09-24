<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { stock } from "../modules/beverage";
  import BeverageDetails from "../lib/BeverageDetails.svelte";

  const params = useParams();

  let beverage = $stock.find((b) => b.id === $params.id);
</script>

<section class="beverage-details">
  <div class="text-2xl mb-8">
    <BeverageDetails {beverage} />
  </div>
  <div class="capabilities">
    {#if beverage.capabilities.isUnit}
      <button class="btn"
        >{$_(
          beverage.metadata.packaging
            ? `beverage.packaging.${beverage.metadata.packaging}`
            : "beverage.capability.unit"
        )}</button
      >
    {/if}
  </div>
</section>
