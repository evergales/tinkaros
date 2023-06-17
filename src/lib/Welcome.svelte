<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { open } from '@tauri-apps/api/dialog';
  import { exists } from '@tauri-apps/api/fs';
  import Tooltip from "../components/Tooltip.svelte";
  import { state } from "../stores/state";
  import { config } from "../stores/config";
  import newToast from "../scripts/toasts";
  
  let launcherList = ["default", "curseforge", "prism", "custom"]
  let knownLaunchers: {name: string, path: string}[];
  let selectedLauncher: string;

  let customPath: string; 

  async function init() {
    $state.loading = true
    var path = selectedLauncher == "custom" && customPath ? customPath : knownLaunchers.find((launcher) => launcher.name == selectedLauncher).path
    var launcher = selectedLauncher == "custom" ? "default" : selectedLauncher
    
    await invoke("init", {chosen: launcher, path: path, custom: selectedLauncher == "custom"}).catch(err => { newToast("error", "unable to write config", err ) })
    await invoke("get_config").then(res => { config.set(res) }).catch(err => { newToast("error", "unable to load config", err ) })
    $state.loading = false
  }

  async function validateCustomOptions() {
    if (await exists(customPath)) {
      document.getElementById("custom-confirm").removeAttribute("disabled")
    } else {
      document.getElementById("custom-confirm").setAttribute("disabled", "")
    }
  }

  function selectLauncher(target: EventTarget & HTMLButtonElement) {
    if (!selectedLauncher) {
      document.getElementById("select-row").setAttribute("style", "top: 40%")
    }

    selectedLauncher = target.id
    target.setAttribute("value", "selected")
    launcherList.forEach((id) => {
      if (target.id != id) {
        document.getElementById(id).removeAttribute("value")
      }
    })
  }

  async function selectPath() {
    var path = await open({directory: true})
    if (Array.isArray(path)) {customPath = path[0]} else customPath = path
    await validateCustomOptions();
  }

  onMount(async () => {
    var known: {name: string, path: string}[] = await invoke("get_launchers")
    var unknown = launcherList.slice(0, 3).filter(function(obj) { return known.map((i) => i.name).indexOf(obj) == -1; });

    unknown.forEach((id) => {
      var element = document.getElementById(id)
      element.setAttribute("disabled", "");
    })
    
    knownLaunchers = known
  })
</script>

<main>
  <div id="select-row">
    <Tooltip tip="default minecraft launcher" top><button on:click={(e) => {selectLauncher(e.currentTarget)}} id="default" class="launcher-button"><img src="./mc_logo.png" alt="default" /></button></Tooltip>
    <Tooltip tip="curseforge launcher" top><button on:click={(e) => {selectLauncher(e.currentTarget)}} id="curseforge" class="launcher-button"><img src="./curseforge.png" alt="curseforge" /></button></Tooltip>
    <Tooltip tip="prism launcher" top><button on:click={(e) => {selectLauncher(e.currentTarget)}} id="prism" class="launcher-button"><img src="./prism.png" alt="prism" /></button></Tooltip>
    <Tooltip tip="custom/any" top><button on:click={(e) => {selectLauncher(e.currentTarget)}} id="custom" class="launcher-button"><span style="font-size: 2.5rem;">?</span></button></Tooltip>
  </div>

  {#if selectedLauncher}
    <div id="confirm-wrapper" transition:fade="{{duration: 200}}">
      {#if selectedLauncher != "custom"}
        <button on:click={init}>confirm</button>
      {:else}
        <li style="list-style: none;">
          <i><input bind:value={customPath} on:change={validateCustomOptions} type="text" placeholder="custom install location"></i>
          <i><button id="select-path" on:click={selectPath}><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H298.5c-17 0-33.3-6.7-45.3-18.7L226.7 50.7c-12-12-28.3-18.7-45.3-18.7H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"/></svg></button></i>
        </li>

        <button id="custom-confirm" on:click={init} disabled>confirm</button>
      {/if}
    </div>
  {/if}
</main>

<style>
  button, input {
    background-color: rgba(0, 0, 0, 0.3)
  }

  #select-row {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%); 
    transition: .5s;
    display: flex;
  }

  #confirm-wrapper {
    position: absolute;
    bottom: 35%;
    left: 50%;
    transform: translate(-50%); 
  }

  #select-path {
    fill: white;
    width: 2.5em;
    height: 2.5em;
    transform: translateY(.25rem);
  }

  #custom-confirm {
    position: absolute;
    bottom: -3rem;
    left: 50%;
    transform: translate(-50%); 
  }

  .launcher-button {
    width: 5rem;
    height: 5rem;
    margin: .2rem;
    transform: scale(95%);
    transition: .2s;
  }

  :global(.launcher-button[value="selected"]) {
    transform: scale(1);
    box-shadow: 0 0 5px 0 rgba(37, 148, 81, 0.2);
  }
</style>