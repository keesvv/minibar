<script lang="ts">
  import { onMount } from "svelte";
  import { stock } from "./modules/stock";
  import { config } from "./modules/config";
  import { api } from "./modules/api";
  import { order } from "./modules/order";
  import { session } from "./modules/auth";
  import { Route, Router, navigate } from "svelte-navigator";
  import Catalog from "./pages/Catalog.svelte";
  import ViewBeverage from "./pages/ViewBeverage.svelte";
  import Login from "./pages/Login.svelte";
  import OrderBar from "./lib/OrderBar.svelte";

  onMount(async () => {
    const authResponse = await api("auth", { throwHttpErrors: false });
    if (authResponse.ok) {
      session.set(await authResponse.json());
    }

    session.subscribe(async (exists) => {
      if (!exists) return navigate("/login");
      config.set(await api("config").json());
      stock.set(await api("beverages").json());
    });

    if (localStorage.getItem("order") != null) {
      order.set(JSON.parse(localStorage.getItem("order")));
    }

    order.subscribe((orderUpdate) => {
      localStorage.setItem("order", JSON.stringify(orderUpdate));
    });
  });
</script>

<div class="app p-10">
  <Router primary={false}>
    <Route path="/"><Catalog /></Route>
    <Route path="/details/:id"><ViewBeverage /></Route>
    <Route path="/login"><Login /></Route>
  </Router>
  {#if $order.length}
    <div class="order-bar-container absolute left-0 bottom-0 right-0">
      <OrderBar />
    </div>
  {/if}
</div>
