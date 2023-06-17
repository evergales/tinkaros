<script async script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { config } from "./stores/config";
  import { BootstrapToast, ToastContainer } from "svelte-toasts"
  import Main from "./lib/Main.svelte";
  import Welcome from "./lib/Welcome.svelte";
  import newToast from "./scripts/toasts";

  onMount(async () => {
    await invoke("get_config").then(async (c) => {
      config.set(c);
    }).catch(err => { newToast("error", "unable to load configs", err) });
    invoke("check_update").then(async (r: [boolean, Object]) => {
      if (r[0]) {       
        let updatePopup = r[1]
        newToast("info", "update available!", "Tinkaros has a new update available it is recommended you update!", 0)
      }
    }).catch(err => { newToast("error", "unable to find tinkaros updates", err) })
  });
</script>

<main>
  <div id="background" class="full"></div>
  <ToastContainer let:data={data}>
    <BootstrapToast theme="dark" {data} />
  </ToastContainer>

  {#if !$config || $config.init == false}
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
