<script lang="ts">
  import { onMount } from "svelte";
  import { stock } from "./modules/stock";
  import { config } from "./modules/config";
  import { api } from "./modules/api";
  import { order } from "./modules/order";
  import { Route, Router } from "svelte-navigator";
  import Catalog from "./pages/Catalog.svelte";
  import ViewBeverage from "./pages/ViewBeverage.svelte";
  import OrderBar from "./lib/OrderBar.svelte";

  onMount(async () => {
    config.set(await api("config").json());
    stock.set(await api("beverages").json());
  });
</script>

<div class="app p-10">
  <Router primary={false}>
    <Route path="/"><Catalog /></Route>
    <Route path="/details/:id"><ViewBeverage /></Route>
  </Router>
  {#if $order.length}
    <div class="order-bar-container absolute left-0 bottom-0 right-0">
      <OrderBar />
    </div>
  {/if}
</div>
