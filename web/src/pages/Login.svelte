<script>
  import { _ } from "svelte-i18n";
  import { navigate } from "svelte-navigator";
  import { onMount } from "svelte";
  import IconContinue from "~icons/line-md/arrow-small-right";
  import { authenticate, session } from "../modules/auth";
  import { config } from "../modules/config";

  let firstName = "";
  $: canLogin = firstName.length > 0;

  let locked = false;

  async function login() {
    const response = await authenticate({ name: firstName });
    locked = response.status === 409;

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
  <form
    class="login-form mb-4 flex gap-2 items-center"
    on:submit|preventDefault={() => login()}
  >
    <input
      class="field"
      type="text"
      placeholder={$_("login.inputs.firstName")}
      bind:value={firstName}
    />
    <button type="submit" class="btn-small" disabled={!canLogin}
      ><IconContinue /></button
    >
  </form>
  {#if locked}
    <span class="text-sm text-gray-500">
      {$_($config.owner ? "login.errors.locked_owner" : "login.errors.locked", {
        values: { owner: $config.owner },
      })}
    </span>
  {/if}
</section>
