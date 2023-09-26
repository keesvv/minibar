<script lang="ts">
  import { P, match } from "ts-pattern";
  import {
    IconShot,
    IconBottle,
    type Beverage,
    type BeverageCapability,
  } from "../modules/beverage";
  import { addToOrder, canOrder, type OrderItem } from "../modules/order";
  import { _ } from "svelte-i18n";

  export let beverage: Beverage;
  export let capability: BeverageCapability;

  let caption = match([beverage, capability])
    .with(
      [{ metadata: { packaging: P.string.and(P.select()) } }, "unit"],
      (pkg) => $_(`beverage.packaging.${pkg}`)
    )
    .otherwise(([, cap]) => $_(`beverage.capability.${cap}`));

  let icon = match([beverage, capability])
    .with([P._, "shot"], () => IconShot)
    .with([{ metadata: { packaging: "bottle" } }, P._], () => IconBottle)
    .otherwise(() => null);

  let orderItem = match<BeverageCapability, OrderItem>(capability)
    .with("shot", () => ({ type: "shot", beverageId: beverage.id }))
    .with("unit", () => ({ type: "unit", beverageId: beverage.id }))
    .otherwise(() => null);
</script>

<button
  class="btn flex gap-2 items-center"
  disabled={!$canOrder || !orderItem}
  on:click={() => addToOrder(orderItem)}
>
  {#if icon}
    <svelte:component this={icon} />
  {/if}
  <span>{caption}</span>
</button>
