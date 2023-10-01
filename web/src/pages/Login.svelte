<script>
  import { _ } from "svelte-i18n";
  import { authenticate, session } from "../modules/auth";
  import { navigate } from "svelte-navigator";
  import { onMount } from "svelte";
  import IconContinue from "~icons/line-md/arrow-small-right";

  let firstName = "";
  $: canLogin = firstName.length > 0;

  async function login() {
    const response = await authenticate({ name: firstName });
    if (!response.ok) {
      return;
    }

    session.set(await response.json());
  }

  onMount(() => {
    session.subscribe((exists) => exists && navigate("/"));
  });
</script>

<section class="login mt-32">
  <h1 class="mb-4 text-2xl">{$_("login.caption")}</h1>
  <div class="login-container mb-4 flex gap-2 items-center">
    <input
      class="field"
      type="text"
      placeholder={$_("login.inputs.firstName")}
      bind:value={firstName}
    />
    <button class="btn-small" disabled={!canLogin} on:click={() => login()}
      ><IconContinue /></button
    >
  </div>
</section>
