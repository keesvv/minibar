<script lang="ts">
  import { P, match } from "ts-pattern";
  import { order, placeOrder } from "../modules/order";
  import { stock, findById } from "../modules/stock";
  import { session } from "../modules/auth";
  import type { Beverage } from "../modules/beverage";
  import { _ } from "svelte-i18n";
  import { press } from "svelte-gestures";
  import IconOrder from "~icons/mdi/send";
  import IconClear from "~icons/mdi/trash";

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
  let clearMode = false;

  function sendOrder() {
    pending = true;

    placeOrder($order)
      .then(() => {
        navigator.vibrate?.([50, 50]);
        order.set([]);
      })
      .finally(() => (pending = false));
  }

  function clearOrder() {
    clearMode = true;
    setTimeout(() => order.set([]), 500);
  }
</script>

<div
  class="order-bar text-white bg-blue-400 dark:bg-blue-500 p-5 rounded-t-md flex gap-2 justify-between items-center"
  class:clear-mode={clearMode}
>
  <span>{orderSummary.join(", ")}</span>
  <button
    class="btn-order p-1 disabled:opacity-50"
    disabled={pending || !$session}
    use:press={{ timeframe: 750, triggerBeforeFinished: true }}
    on:press={() => clearOrder()}
    on:click={() => sendOrder()}
  >
    {#if clearMode}
      <IconClear class="text-2xl" />
    {:else}
      <IconOrder class="text-2xl" />
    {/if}
  </button>
</div>

<style scoped lang="postcss">
  .clear-mode {
    @apply bg-red-400 dark:bg-red-500;
  }
</style>
