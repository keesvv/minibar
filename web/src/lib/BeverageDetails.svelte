<script lang="ts">
  import { IconShot, type Beverage } from "../modules/beverage";
  import { _ } from "svelte-i18n";
  import { config } from "../modules/config";
  import IconBeer from "~icons/line-md/beer-twotone-loop";

  export let beverage: Beverage;

  let details = [
    beverage.metadata.category &&
      $_(`beverage.category.${beverage.metadata.category}`),
    beverage.metadata.alcPercent &&
      `${(beverage.metadata.alcPercent * 100).toLocaleString()}%`,
  ]
    .filter((p) => p)
    .join(", ");

  let remaining = beverage.amount * beverage.capacity;
  let shotsRemaining = remaining / $config.size.shot;
  let almostOut = remaining < beverage.capacity / 3;
</script>

<div class="beverage-details">
  <h1>{beverage.description}</h1>
  <div class="text-gray-400">
    {#if details}
      <h2 class="mb-1.5">{details}</h2>
    {/if}
    <h2 class="flex gap-2">
      <div class="remaining flex items-center gap-1">
        <span class:text-red-500={almostOut}>
          <IconBeer />
        </span>
        <span>{(remaining * 1000).toLocaleString()}L</span>
      </div>
      {#if beverage.capabilities.includes("shot")}
        <div class="remaining-shots flex items-center gap-1">
          <IconShot />
          <span>{Math.floor(shotsRemaining)}</span>
        </div>
      {/if}
    </h2>
  </div>
</div>
