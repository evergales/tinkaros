<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { exists } from '@tauri-apps/api/fs';
  import { open } from '@tauri-apps/api/dialog';
  import { config } from "../stores/config";
  import { state } from "../stores/state";
  import Tooltip from "./Tooltip.svelte";
  const all = ["default", "curseforge", "prism", "custom"]
  let selected: string;
  let customSelected: string
  let path: string | string[];
  let validPath: boolean = true;
  let launchers: { name: string; path: string; }[];

  function select(e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement; }) {
    if (selected == undefined) {
      document.getElementById("selectDiv").setAttribute("style", "top: 45%;")
    }
    all.forEach(e => {
          let _e = document.getElementById(e)
          _e.removeAttribute("value")
    })
    e.currentTarget.setAttribute("value", "selected")
    selected = e.currentTarget.id
  }

  async function selectPath() {
    path = await open({directory: true})
    if (Array.isArray(path)) {path = path[0]}
  }

  async function init(e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement; }) {
    $state.loading = true
    all.forEach(e => {
          let _e = document.getElementById(e)
          _e.removeAttribute("value")
    })
    await invoke("init", {
    chosen: selected == "custom" ?
      customSelected: selected,
    path: selected == "custom" ?
      !path ? "" : path : launchers.find((i) => i.name == selected).path,
    custom: selected == "custom" ?
      true : false
  })
    await invoke("get_config").then((res) => { config.set(res) })
    $state.loading = false
  }

  async function get_known() {
    let known: Array<{name: string, path: string}> = await invoke("get_launchers")
    let unknown = all.slice(0, 3).filter(function(obj) { return known.map((i) => i.name).indexOf(obj) == -1; });
    if (unknown.length > 0) {
      unknown.forEach((e) => {
        let _e = document.getElementById(e)
        _e.setAttribute("disabled", "true");
        _e.removeAttribute("value")
      })
    }
    launchers = known
  }

  async function valid_path() {
    if (Array.isArray(path)) {path = path[0]}
    let known = await exists(path)
    if (!known && path != "") {
      validPath = false
    } else {
      validPath = true
    }
  }

  get_known()
</script>

<main>
  <div id="selectDiv" class="row; selectLauncher">    
    <Tooltip tip="default minecraft launcher" top><button on:click={(e) => {select(e)}} id="default"><img src="./mc_logo.png" alt="default" /></button></Tooltip>
    <Tooltip tip="curseforge launcher" top><button on:click={(e) => {select(e)}} id="curseforge"><img src="./curseforge.png" alt="curseforge" /></button></Tooltip>
    <Tooltip tip="prism launcher" top><button on:click={(e) => {select(e)}} id="prism"><img src="./prism.png" alt="prism" /></button></Tooltip>
    <Tooltip tip="custom/any" top><button on:click={(e) => {select(e)}} id="custom" style="transform: translateY(0.6em);"><img src="./custom.png" alt="prism" style="transform: translateY(0.3em);"/></button></Tooltip>
  </div>
  {#if selected != "custom" && selected != undefined}
    <button on:click={(e) => init(e)} class="confirmButton bottomEl">confirm selection</button>
  {:else if selected == "custom"}
    <div class="customDiv bottomEl">
      <li style="list-style: none;">
        <i><input bind:value={path} on:change={valid_path} type="text" placeholder="custom install location"></i>
        <i><button class="selectPath" on:click={selectPath}><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H298.5c-17 0-33.3-6.7-45.3-18.7L226.7 50.7c-12-12-28.3-18.7-45.3-18.7H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"/></svg></button></i>
      </li>
      <select bind:value={customSelected}>
        <option value="default">default</option>
        <option value="curseforge">curseforge</option>
        <option value="prism">prism</option>
      </select>
      {#if validPath}
      <button on:click={(e) => init(e)} class="customConfirm">confirm</button>  
      {:else}
      <Tooltip tip="install location invalid!" top><button class="customConfirm" style="transform: translateX(0%);" disabled>confirm</button></Tooltip>
      {/if}
      
    </div>
  {/if}
</main>

<style>
  .selectLauncher {
    transform: translateX(-50%) translateY(-50%);
    top: 50%;
    left: 50%;
    position: absolute;
    transition: 0.2s;
  }
  .selectLauncher button {
    width: 6em;
    height: 6em;
    transition: 0.2s;
  }
  .selectLauncher :global(button[value="selected"]) {
    transform: scale(105%);
    border-color: #1c7841;
  }
  .selectLauncher button:disabled {
    background-color: rgb(59, 59, 59);
    cursor: default;
  }
  .customDiv {
    display: grid;
    gap: 1em;
  }
  .selectPath {
    margin: 0;
    padding: 0.5em;
    width: 2.25em;
    height: 2.25em;
    transform: translateY(0.15em);
  }
  .selectPath svg {
    fill: white;
    width: 1em;
    height: 1em;
  }
  .confirmButton {
    padding: 1em;
    background-color: #0d3b20;
  }
  .customConfirm {
    padding: 0.8em;
    background-color: #0d3b20;
    width: 50%;
    transform: translateX(50%);
  }
  .customConfirm:disabled {
    background-color: rgb(59, 59, 59);
    cursor: default;
  }
  .customConfirm:disabled:hover {
      border-color: rgb(59, 59, 59);
    }
  .bottomEl {
    position: absolute;
    transform: translateX(-50%) translateY(-50%);
    top: 70%;
    left: 50%;
  }
  option {
    background-color: #0f0f0fb9;
    
  }
</style>
