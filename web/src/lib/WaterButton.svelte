<script lang="ts">
  import { _ } from "svelte-i18n";
  import { addToOrder, canOrder, order } from "../modules/order";
  import IconWater from "~icons/mdi/water";

  $: canAdd = !$order.some((item) => item.type === "water");
</script>

<button
  class="btn-alt flex gap-2 items-center"
  disabled={!canAdd || !$canOrder}
  on:click={() => addToOrder({ type: "water" })}
>
  <IconWater />
  <span>{$_("water")}</span>
</button>
