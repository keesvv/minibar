<script lang="ts">
  import { P, match } from "ts-pattern";
  import { order, placeOrder } from "../modules/order";
  import IconOrder from "~icons/mdi/send";
  import { _ } from "svelte-i18n";
  import { stock, findById } from "../modules/beverage";

  $: orderSummary = $order.map((ord) =>
    match(ord)
      .with({ type: "water" }, () => $_("water"))
      .with({ type: "mix" }, (item) =>
        item.beverageIds.map((id) => findById($stock, id).description).join(" ")
      )
      .with(
        { type: "shot", beverageId: P.select() },
        (id) =>
          `${findById($stock, id).description} ${$_(
            "beverage.capability.shot"
          )}`
      )
      .with(
        { type: "unit", beverageId: P.select() },
        (id) => findById($stock, id).description
      )
      .run()
  );
</script>

<div
  class="order-bar text-white bg-blue-400 dark:bg-blue-500 p-5 rounded-t-md flex gap-2 justify-between items-center"
>
  <span>{orderSummary.join(", ")}</span>
  <button class="btn-order p-1" on:click={() => placeOrder()}>
    <IconOrder class="text-2xl" />
  </button>
</div>
