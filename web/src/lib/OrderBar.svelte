<script lang="ts">
  import { match } from "ts-pattern";
  import { order, placeOrder } from "../modules/order";
  import IconOrder from "~icons/mdi/send";
  import { _ } from "svelte-i18n";
  import { stock } from "../modules/beverage";

  $: orderSummary = $order.map((ord) =>
    match(ord)
      .with({ type: "water" }, () => $_("water"))
      .with({ type: "mix" }, (item) =>
        item.beverageIds
          .map((id) => $stock.find((s) => s.id === id).description)
          .join(" ")
      )
      .run()
  );
</script>

<div
  class="order-bar text-white bg-blue-400 dark:bg-blue-500 p-6 rounded-t-md flex gap-2 justify-between items-center"
>
  <span>{orderSummary.join(", ")}</span>
  <button class="btn-order" on:click={() => placeOrder()}>
    <IconOrder class="text-2xl" />
  </button>
</div>
