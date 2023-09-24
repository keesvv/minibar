<script lang="ts">
  import type { Beverage } from "../modules/beverage";
  import { _ } from "svelte-i18n";
  import IconBeer from "~icons/line-md/beer-twotone-loop";

  export let beverage: Beverage;

  let remaining: number;
  $: remaining = beverage.amount * beverage.capacity;

  let details: string;
  $: details = [
    beverage.metadata.category &&
      $_(`beverage.category.${beverage.metadata.category}`),
    beverage.metadata.alcPercent &&
      `${(beverage.metadata.alcPercent * 100).toLocaleString()}%`,
  ]
    .filter((p) => p)
    .join(", ");

  let almostOut = false;
  $: almostOut = remaining < beverage.capacity / 3;
</script>

<div class="beverage-tile bg-white p-6 md:max-w-xs shadow-md rounded-md flex">
  {#if beverage.metadata.imageUri}
    <img
      class="h-full w-20 max-h-24 mr-5 object-contain"
      src={beverage.metadata.imageUri}
      alt={beverage.description}
    />
  {/if}
  <div>
    <h1 class="text-xl">{beverage.description}</h1>
    <div class="text-gray-400">
      {#if details}
        <h2 class="text-md">{details}</h2>
      {/if}
      <h2 class="flex items-center">
        <span class="mr-1" class:text-red-500={almostOut}>
          <IconBeer />
        </span>
        <span>{(remaining * 1000).toLocaleString()}L</span>
      </h2>
    </div>
  </div>
</div>
