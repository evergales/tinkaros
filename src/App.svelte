<script async script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { config } from "./stores/config";
  import { state } from "./stores/state";
  import Loading from "./components/Loading.svelte";
  import Main from "./lib/Main.svelte";
  import Welcome from "./lib/Welcome.svelte";

  onMount(async () => {
    await invoke("get_config").then(async (c) => {
      config.set(c);
    });
    invoke("check_update").then(async (r: [boolean, Object]) => {
      if (r[0]) {       
        let updatePopup = r[1]
        updatePopup["shown"] = true
        $state.updatePopup = updatePopup
      }
    })
    $state.loading = false
  });
</script>

<main>
  <div id="background" class="full"></div>

  {#if $state.loading}
    <Loading />
  {:else if !$config || $config.init == false}
    <Welcome />
  {:else}
    <Main />
  {/if}
</main>

<style>
  #background {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: -1;

    background-image: url("https://github.com/Hbarniq/ahms/raw/main/assets/backgrounds/3.png");
    background-size: cover;
    filter: blur(3px) brightness(80%);
  }
</style>
