<script lang="ts">
  import { P, match } from "ts-pattern";
  import { order, placeOrder } from "../modules/order";
  import IconOrder from "~icons/mdi/send";
  import { _ } from "svelte-i18n";
  import { stock, findById } from "../modules/stock";
  import { session } from "../modules/auth";
  import type { Beverage } from "../modules/beverage";

  function findFallback(stock: Beverage[], itemId: Beverage["id"]): Beverage {
    return (
      findById(stock, itemId) ?? {
        id: itemId,
        description: $_("beverage.unknown"),
        capacity: 0,
        amount: 0,
        metadata: {},
        capabilities: [],
      }
    );
  }

  $: orderSummary = $order.map((ord) =>
    match(ord)
      .with({ type: "water" }, () => $_("water"))
      .with({ type: "mix" }, (item) =>
        item.beverageIds
          .map((id) => findFallback($stock, id).description)
          .join(" ")
      )
      .with(
        { type: "shot", beverageId: P.select() },
        (id) =>
          `${findFallback($stock, id).description} ${$_(
            "beverage.capability.shot"
          )}`
      )
      .with(
        { type: "unit", beverageId: P.select() },
        (id) => findFallback($stock, id).description
      )
      .run()
  );

  let pending = false;

  function sendOrder() {
    pending = true;

    placeOrder($order)
      .then(() => {
        navigator.vibrate?.([50, 50]);
        order.set([]);
      })
      .finally(() => (pending = false));
  }
</script>

<div
  class="order-bar text-white bg-blue-400 dark:bg-blue-500 p-5 rounded-t-md flex gap-2 justify-between items-center"
>
  <span>{orderSummary.join(", ")}</span>
  <button
    class="btn-order p-1 disabled:opacity-50"
    disabled={pending || !$session}
    on:click={() => sendOrder()}
  >
    <IconOrder class="text-2xl" />
  </button>
</div>
