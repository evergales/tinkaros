<script lang="ts">
  import { config } from "../stores/config";
  import { state } from "../stores/state";
  import { invoke } from "@tauri-apps/api/tauri";
  import Tooltip from "../components/Tooltip.svelte";
  import Progress from "../components/Progress.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { timeSince } from "../scripts/parseTime";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  
  let bg: string;
  let initial: boolean;
  let done: boolean = false;
  
  let updatedAt: string = "...";
  let currentVer: string = "...";
  let latestVer: string = "...";
  let lastUpdated: number;
  
  async function getinfo() {
    let max = 5;
    let chosen = Math.floor(Math.random() * max + 1);
    bg = `https://github.com/Hbarniq/ahms/raw/main/assets/backgrounds/${chosen}.png`;
    initial = await invoke("check_installed", { path: $config.path }) == true ? false : true
    version()
  }
  
  async function update() {
    $state.updating = true
    await invoke("update", { launcher: $config.launcher, path: $config.path, custom: $config.custom })
    await invoke("log_update", { path: $config.path })
    done = true
  }

  async function finish() {
    version()
    initial = false
    $state.updating = false
    $state.updateState = "waiting.."
    $state.progress = 0
    done = false
  }

  async function version() {
    await invoke("get_version", { path: $config.path }).then(async (res: any) => {
      lastUpdated = res.last_updated;
      currentVer = res.version;
      latestVer = res.latest_version;
    })
    updateTime()
  }

  async function updateTime() {
    updatedAt = lastUpdated == 0 ? "never" : timeSince(lastUpdated)
  }

  onMount(() => {
    getinfo();
    setInterval(() => { updateTime() }, 30000)
    listen("status", (event: any) => {
      $state.updateState = event.payload.status
    })

    listen("progressUpdate", (event: any) => {
      $state.progress = event.payload.progress
    })
  })
</script>

<!-- svelte-ignore a11y-missing-attribute -->
<main>
  <div class="bg-img" style="background-image: url({bg});">
        <div class="update-box" in:fly={{duration: 500, y: -1000, easing: cubicOut}}>
            <h1>{$config.custom ? `custom | ${$config.launcher}` : $config.launcher}</h1>
            <div class="update" style={$state.updating ? 'background-color: #0f0f0f98; border-radius: 8px;' : ''}>
                {#if $state.updating}
                <a>{$state.updateState}</a><br>
                <a>progress: {$state.progress}%</a>
                <Progress />
                {#if done}<button on:click={finish} style="background-color: #0d3b20;">close</button>{/if}
                {:else}
                {#if currentVer != latestVer && currentVer != "not installed"}
                    <a style="color: orange;">update available!</a>
                {/if}
                <div style="display: grid; background-color: #0f0f0f98; border-radius: 8px; padding: 1em;">
                  <a>last updated: {updatedAt}</a>
                  <a>installed: {currentVer}</a>
                  <a>latest: {latestVer}</a>
                </div>
                  <button on:click={update} style="padding: 1em; background-color: #0d3b20; transform: translateY(0.5em);">{initial ? "install" : "update"}</button>
                {/if}
            </div>
            
            <div class="openPath">
              <Tooltip tip="open path" top><button on:click={() => invoke("explorer", { path: $config.path })}><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H298.5c-17 0-33.3-6.7-45.3-18.7L226.7 50.7c-12-12-28.3-18.7-45.3-18.7H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"/></svg></button></Tooltip>
            </div>
        </div>
    </div>
</main>

<style>
  .update {
    transform: translateX(-50%) translateY(-50%);
    top: 50%;
    left: 50%;
    width: max-content;
    position: absolute;
    display: flexbox;
    padding: 2em;
  }
  .bg-img {
    transform: translateX(-50%) translateY(-50%);
    top: 50%;
    left: 50%;
    position: absolute;
    width: 100%;
    height: 100%;
    background-size: cover;
  }
  .update-box {
    display: flexbox;
    transform: translateX(-50%) translateY(-50%);
    top: 50%;
    left: 50%;
    position: absolute;
    background-color: #2b2b2b;
    padding: 2em;
    width: 34em;
    height: 20em;
    border-radius: 1.5em;
    box-shadow: 0 0.2em rgba(0, 0, 0, 0.2) rgba(0, 0, 0, 0.2);
  }
  .openPath {
    position: absolute;
    bottom: 1em;
    right: 1em;
    width: 2.5em;
    height: 2.5em;
  }
  .openPath svg {
    fill: white;
    width: 1em;
    height: 1em;
  }
</style>
