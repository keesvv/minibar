<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { _ } from "svelte-i18n";
  import { stock, IconShot, IconBottle } from "../modules/beverage";
  import BeverageDetails from "../lib/BeverageDetails.svelte";
  import BeverageButton from "../lib/BeverageButton.svelte";
  import type { SvelteComponent } from "svelte";
  import { P, match } from "ts-pattern";

  const params = useParams();

  let beverage = $stock.find((b) => b.id === $params.id);
  let softdrinks = $stock.filter((s) => s.metadata.category === "softdrink");

  let shownCapabilities = beverage.capabilities.filter((c) =>
    ["unit", "shot"].includes(c)
  );

  let capabilityMeta = shownCapabilities.map((cap) => {
    let caption = match([beverage, cap])
      .with(
        [{ metadata: { packaging: P.string.and(P.select()) } }, "unit"],
        (pkg) => $_(`beverage.packaging.${pkg}`)
      )
      .otherwise(() => $_(`beverage.capability.${cap}`));

    let icon = match([beverage, cap])
      .with([P._, "shot"], () => IconShot)
      .with([{ metadata: { packaging: "bottle" } }, P._], () => IconBottle)
      .otherwise(() => null);

    return {
      id: cap,
      caption,
      icon,
    };
  });
</script>

<section class="beverage-details">
  <div class="text-2xl mb-8">
    <BeverageDetails {beverage} />
  </div>
  <div class="capabilities grid gap-3">
    {#each capabilityMeta as capability}
      <button class="btn flex gap-2 items-center">
        {#if capability.icon}
          <svelte:component this={capability.icon} />
        {/if}
        <span>{capability.caption}</span>
      </button>
    {/each}
  </div>
  {#if beverage.capabilities.includes("mix")}
    <div class="mixWith mt-3 grid gap-3">
      {#each softdrinks as softdrink}
        <button>
          <BeverageButton beverage={softdrink} />
        </button>
      {/each}
    </div>
  {/if}
</section>
