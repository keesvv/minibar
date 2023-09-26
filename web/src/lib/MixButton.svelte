<script lang="ts">
  import BeverageInfo from "./BeverageInfo.svelte";
  import { addToOrder, canOrder } from "../modules/order";
  import type { Beverage } from "../modules/beverage";

  export let beverage: Beverage;
  export let softdrink: Beverage;
</script>

<button
  disabled={!$canOrder}
  class="mix-button btn text-left"
  on:click={() =>
    addToOrder({
      type: "mix",
      beverageIds: [beverage.id, softdrink.id],
    })}
>
  <BeverageInfo beverage={softdrink} />
</button>
