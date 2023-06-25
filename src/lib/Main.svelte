<script lang="ts">
  import { onMount } from "svelte";
  import { state } from "../stores/state";
  import { config } from "../stores/config";
  import { slide } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { timeSince } from "../scripts/parseTime";
  import UpdatePopup from "../components/updatePopup.svelte";
  import Progress from "../components/Progress.svelte";
  import ModCard from "../components/ModCard.svelte";
  import ChangelogCard from "../components/ChangelogCard.svelte";
  import newToast from "../scripts/toasts";
  import Loading from "../components/Loading.svelte";
  
  let initial = false
  let lastUpdated: string | null = null
  let isUpdated = false
  let modlist: Mod[] = []
  let changelog: { version: string | undefined, description: string | undefined }[] = []

  async function update() {
    var button = document.getElementById("update-button")
    button.setAttribute("disabled", "")
    $state.updating = true

    try {
      await invoke("update", { launcher: $config.launcher, path: $config.path })
    } catch (err) {
      finishUpdate()
      return newToast("error", "error while updating", err );
    }

    await invoke("log_update", { path: $config.path }).catch(err => { newToast("error", "error while logging update", err ) })
    updateVersion()
    finishUpdate()
  }

  function finishUpdate() {
    var button = document.getElementById("update-button")
    setTimeout(() => {
      button.removeAttribute("disabled")
      $state.progress = 0
      $state.updating = false
      initial = false
    }, 5000);
  }

  async function updateVersion() {
    await invoke("get_version", { path: $config.path }).catch(err => { newToast("error", "error while trying to fetch version", err) }).then(async (res: any) => {
      lastUpdated = res.last_updated == 0 ? null : timeSince(res.last_updated);
      isUpdated = res.version != res.latest_version ? false : true
    })
  }

  onMount(async () => {
    updateVersion()
    initial = await invoke("check_modpack_installed", { path: $config.path }).catch(err => { newToast("error", undefined, err) }) == true ? false : true

    listen("status", (event: any) => {
      $state.updateState = event.payload.status
    })

    listen("progressUpdate", (event: any) => {
      $state.progress = event.payload.progress
    })

    invoke("list_mod_projects").then((res: Mod[]) => modlist = res).catch(err => { newToast("error", "unable to list mods", err) })
    changelog = JSON.parse(await (await fetch("https://gist.githubusercontent.com/Hbarniq/86838647fb0cc4dce6913d2d73ce7fc4/raw/ahms-changelog.json")).text())
  })
</script>
<main>
  {#if $state.updatePopup.shown}
    <UpdatePopup />
  {/if}

  <div id="layout" class="layout-fullscreen">
    <div class="container">
      {#if !$state.updating}
      <div id="update-info" transition:slide="{{duration: 200}}">
        <p style="margin: 0; line-height: 1rem;">{isUpdated ? "up to date!" : "outdated version!"}</p>
        {#if lastUpdated != null}
        <p transition:slide="{{duration: 200}}" style="margin: 0; line-height: .7rem;"><span style="color: rgba(255, 255, 255, .6);">updated:</span><br>{lastUpdated}</p>
        {/if}
      </div>
      {/if}

      {#if $state.updating}
      <p transition:slide="{{duration: 500}}" style="margin: 0.2rem; font-size: 0.8rem; max-width: 75%;">
        { $state.updateState }
      </p>

      <Progress />
      {/if}
      
      <button on:click={update} id="update-button">{initial ? "install" : "update"}</button>

      <div id="layout-buttons">
        <button><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M0 416c0 17.7 14.3 32 32 32l54.7 0c12.3 28.3 40.5 48 73.3 48s61-19.7 73.3-48L480 448c17.7 0 32-14.3 32-32s-14.3-32-32-32l-246.7 0c-12.3-28.3-40.5-48-73.3-48s-61 19.7-73.3 48L32 384c-17.7 0-32 14.3-32 32zm128 0a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zM320 256a32 32 0 1 1 64 0 32 32 0 1 1 -64 0zm32-80c-32.8 0-61 19.7-73.3 48L32 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l246.7 0c12.3 28.3 40.5 48 73.3 48s61-19.7 73.3-48l54.7 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-54.7 0c-12.3-28.3-40.5-48-73.3-48zM192 128a32 32 0 1 1 0-64 32 32 0 1 1 0 64zm73.3-64C253 35.7 224.8 16 192 16s-61 19.7-73.3 48L32 64C14.3 64 0 78.3 0 96s14.3 32 32 32l86.7 0c12.3 28.3 40.5 48 73.3 48s61-19.7 73.3-48L480 128c17.7 0 32-14.3 32-32s-14.3-32-32-32L265.3 64z"/></svg></button>
        <button on:click={e => { invoke("explorer", { path: $config.path }) }}><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M88.7 223.8L0 375.8V96C0 60.7 28.7 32 64 32H181.5c17 0 33.3 6.7 45.3 18.7l26.5 26.5c12 12 28.3 18.7 45.3 18.7H416c35.3 0 64 28.7 64 64v32H144c-22.8 0-43.8 12.1-55.3 31.8zm27.6 16.1C122.1 230 132.6 224 144 224H544c11.5 0 22 6.1 27.7 16.1s5.7 22.2-.1 32.1l-112 192C453.9 474 443.4 480 432 480H32c-11.5 0-22-6.1-27.7-16.1s-5.7-22.2 .1-32.1l112-192z"/></svg></button>
      </div>
    </div>

    <div class="container">
      {#if changelog.length == 0}
        <Loading />
      {:else}
        {#each changelog as log}
        <ChangelogCard log={log} />
        {/each}
      {/if}
    </div>
    <div class="container">
      {#if modlist.length == 0}
        <Loading />
      {:else}
        {#each modlist as mod}
        <ModCard any_mod={mod} />
        {/each}
      {/if}
    </div>
  </div>
  
</main>

<style>
  .layout-fullscreen {
    position: absolute;
    top: 50%;
    left: 50%;
    height: 100vh;
    width: 100vw;
    transform: translate(-50%, -50%);
    box-sizing: border-box;
    overflow: hidden;
    padding: .7rem;
  }

  #layout {
    max-width: 1000px;
    max-height: 600px;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-gap: 0.7rem;
  }

  #layout .container { position: relative; }
  
  #layout > :first-child {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;

    height: 50%;
    margin-top: 45%;
  }

  #layout > :not(:first-child) {
    overflow-y: scroll;
  }
  
  #layout-buttons {
    position: absolute;
    display: flex;
    gap: 1rem;
    left: 50%;
    bottom: -15vh;
    transform: translate(-50%);
  }

  #layout-buttons > button {
    display: flex;
    align-items: center;
    justify-content: center;
    fill: rgba(255, 255, 255, .8);
    width: 2.5rem;
    height: 2.5rem;
  }

  #layout-buttons > button > svg {
    width: 1.2rem;
    height: 1.2rem;
  }

  #update-info {
    position: absolute;
    top: 0;
    border-bottom-left-radius: 5px;
    border-bottom-right-radius: 5px;
    font-size: 0.7rem;
    max-width: max-content;
    padding: 0.5rem;
    background-color: rgba(0, 0, 0, 0.2);
  }

  #update-button {
    padding: .6rem;
    font-size: .8rem;
  }
</style>