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
  
  let initial = false
  let lastUpdated: string | null = null
  let isUpdated = false
  let modlist: Mod[] = []
  let changelog: { version: string | undefined, description: string | undefined }[] = []

  async function update() {
    var button = document.getElementById("update-button")
    button.setAttribute("disabled", "")
    $state.updating = true
    await invoke("update", { launcher: $config.launcher, path: $config.path, custom: $config.custom }).catch(err => { newToast("error", "error while updating", err ) })
    await invoke("log_update", { path: $config.path }).catch(err => { newToast("error", "error while logging update", err ) })
    updateVersion()
    setTimeout(() => {
      button.removeAttribute("disabled")
      $state.progress = 0
      $state.updating = false
      initial = false
    }, 5000);
  }

  async function updateVersion() {
    await invoke("get_version", { path: $config.path }).catch(err => { newToast("error", "error while logging update", err) }).then(async (res: any) => {
      lastUpdated = res.last_updated == 0 ? null : timeSince(res.last_updated);
      isUpdated = res.version != res.latest_version ? false : true
    })
  }

  onMount(async () => {
    updateVersion()
    setInterval(() => { updateVersion() }, 30000)
    initial = await invoke("check_installed", { path: $config.path }).catch(err => { newToast("error", undefined, err) }) == true ? false : true

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
        <div class="container" style="padding: 1rem;"></div>
        <div class="container" style="padding: 1rem;"></div>
      </div>
    </div>

    <div class="container">
      {#each changelog as log}
        <ChangelogCard log={log} />
      {/each}
    </div>
    <div class="container">
      {#each modlist as mod}
        <ModCard any_mod={mod} />
      {/each}
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