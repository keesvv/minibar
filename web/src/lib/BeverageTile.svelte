<script lang="ts">
  import { getCategoryName } from "../modules/beverage";
  import type { Beverage } from "../modules/beverage";

  export let beverage: Beverage;

  let remaining: number;
  $: remaining = beverage.amount * beverage.capacity;

  let details: string;
  $: details = [
    beverage.metadata.category && getCategoryName(beverage),
    beverage.metadata.alcPercent &&
      `${(beverage.metadata.alcPercent * 100).toLocaleString()}%`,
  ]
    .filter((p) => p)
    .join(", ");

  let almostOut = false;
  $: almostOut = remaining < beverage.capacity / 3;
</script>

<div class="beverage-tile p-8 max-w-xs shadow-md rounded-md">
  {#if beverage.metadata.imageUri}
    <img
      class="w-10 mx-auto mb-5"
      src={beverage.metadata.imageUri}
      alt={beverage.description}
    />
  {/if}
  <h1 class="text-xl">{beverage.description}</h1>
  <div class="flex justify-between text-gray-400">
    {#if details}
      <h2 class="text-md">{details}</h2>
    {/if}
    <h2 class:text-red-500={almostOut}>
      {(remaining * 1000).toLocaleString()}L
    </h2>
  </div>
</div>
