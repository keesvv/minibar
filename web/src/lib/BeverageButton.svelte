<script lang="ts">
  import type { Beverage } from "../modules/beverage";
  import { Link } from "svelte-navigator";
  import BeverageDetails from "./BeverageDetails.svelte";

  export let beverage: Beverage;
</script>

<Link
  to="/details/{beverage.id}"
  class="beverage-tile bg-white dark:bg-gray-800 p-6 md:max-w-xs shadow-md rounded-md flex text-left items-center"
>
  {#if beverage.metadata.imageUri}
    <img
      class="h-full w-20 max-h-24 mr-5 object-contain"
      src={beverage.metadata.imageUri}
      alt={beverage.description}
    />
  {/if}
  <BeverageDetails {beverage} />
</Link>
