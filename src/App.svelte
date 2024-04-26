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
    if ($config.check_tinkaros_update) {
      invoke("check_tinkaros_update").then(async (r: boolean) => {
        if (r) {
          newToast("info", "update available!", "Tinkaros has a new update available it is recommended you update!", 15000, () => { open("https://github.com/evergales/tinkaros/releases/latest") })
        }
      }).catch(err => { newToast("error", "unable to find tinkaros updates", err) })
    }
    $state.loading = false
  });
</script>

<main>
  <div id="background"></div>
  <ToastContainer let:data={data}>
    <BootstrapToast theme="dark" {data} />
  </ToastContainer>

  {#if $state.loading}
    <div class="blur-background" transition:fade="{{duration: 200}}">
      <Loading />
    </div>
  {/if}

  {#if $config && $config.init}
    <Main />
  {:else}
    <Welcome />
  {/if}
</main>

<style>
  #background {
    position: absolute;
    width: 100%;
    height: 100%;
    z-index: -1;

    background: radial-gradient(
      rgba(255, 255, 255, .2) 8%,
      transparent 8%
    );
    background-position: 0% 0%;
    background-size: 2rem 2rem;

    animation: shift-bg 50s linear infinite;
  }

  @keyframes shift-bg {
    from { background-position: 0 0;}
    to { background-position: 100vw 100vw;}
  }
</style>
