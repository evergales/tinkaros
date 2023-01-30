<script async script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { config } from "./stores/config";
  import { state } from "./stores/state";
  import Loading from "./components/Loading.svelte";
  import Main from "./lib/Main.svelte";
  import Welcome from "./lib/Welcome.svelte";

  async function init() {
    await invoke("get_config").then(async (c) => {
      config.set(c);
    });
    $state.loading = false
  }

  onMount(async () => {
    init();
  });
  // todo! selecting path for ahms launcher and other launchers
</script>

<main class="container">
  {#if $state.loading}
    <Loading />
  {:else if !$config || $config.init == false}
    <Welcome />
  {:else}
    <Main />
  {/if}
</main>
