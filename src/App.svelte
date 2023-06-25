<script async script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { config } from "./stores/config";
  import { BootstrapToast, ToastContainer } from "svelte-toasts"
  import { open } from '@tauri-apps/api/shell';
  import Main from "./lib/Main.svelte";
  import Welcome from "./lib/Welcome.svelte";
  import newToast from "./scripts/toasts";
  import Loading from "./components/Loading.svelte";
  import { state } from "./stores/state";
  import { fade } from "svelte/transition";

  onMount(async () => {
    try { await invoke("check_online"); } catch (err) {
      return newToast("error", "internet not connected", err);
    }

    await invoke("get_config").then(async (c) => {
      config.set(c);
    }).catch(err => { newToast("error", "unable to load configs", err) });
    invoke("check_tauri_update").then(async (r: boolean) => {
      if (r) {
        newToast("info", "update available!", "Tinkaros has a new update available it is recommended you update!", 15000, () => { open("https://github.com/Hbarniq/tinkaros/releases/latest") })
      }
    }).catch(err => { newToast("error", "unable to find tinkaros updates", err) })
    $state.loading = false
  });
</script>

<main>
  <div id="background" class="full"></div>
  <ToastContainer let:data={data}>
    <BootstrapToast theme="dark" {data} />
  </ToastContainer>

  {#if $state.loading}
    <div id="loading-background" transition:fade="{{duration: 200}}">
      <Loading />
    </div>
  {/if}

  {#if !$config || $config.init == false}
    <Welcome />
  {:else}
    <Main />
  {/if}
</main>

<style>
  #loading-background {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 2;
    background: rgba(48, 48, 48, 0.6) !important;
    backdrop-filter: blur(5px);
    -webkit-backdrop-filter: blur(5px);
  }

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
