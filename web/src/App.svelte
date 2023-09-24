<script lang="ts">
  import { onMount } from "svelte";
  import { stock, type Beverage } from "./modules/beverage";
  import { config } from "./modules/config";
  import { api } from "./modules/api";
  import { Route, Router } from "svelte-navigator";
  import Catalog from "./pages/Catalog.svelte";
  import ViewBeverage from "./pages/ViewBeverage.svelte";

  onMount(async () => {
    config.set(await api("config").json());
    stock.set(await api("beverages").json());
  });
</script>

<div class="app p-10">
  <Router>
    <Route path="/" component={Catalog} />
    <Route path="/details/:id" component={ViewBeverage} />
  </Router>
</div>
