<script lang="ts">
  import { useParams } from "svelte-navigator";
  import { stock } from "../modules/beverage";
  import BeverageDetails from "../lib/BeverageDetails.svelte";

  const params = useParams();

  let beverage = $stock.find((b) => b.id === $params.id);
</script>

<section class="beverage-details">
  <div class="text-2xl">
    <BeverageDetails {beverage} />
  </div>
</section>
