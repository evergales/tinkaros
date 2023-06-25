<script lang="ts">
  import { state } from "../stores/state";
  import { config } from "../stores/config";
  import { onMount } from "svelte";
  import { tippy } from "svelte-tippy"
  import { fade } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/tauri";
  import newToast from "../scripts/toasts";

  let temp_settings: config = {}

  async function save() {
    $config = temp_settings
    invoke("write_config", { config: temp_settings }).catch(err => { newToast("error", "unable to update config", err) })
  }

  onMount(() => {
    temp_settings = $config
  })
</script>
<main>
  <div class="blur-background" transition:fade="{{ duration: 200 }}">
    <div id="settings-modal">
      <button use:tippy={{ content: "close & save" }} id="close-button" on:click={e => { $state.settingsShown = false; save(); }}><svg xmlns="http://www.w3.org/2000/svg" height="1.2rem" viewBox="0 0 384 512"><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z"/></svg></button>
      <div class="settings-option">
        <span use:tippy={{ content: "whether to check if new updates are available for tinkaros on startup" }} >check for tinkaros updates</span>
        <label class="switch">
          <input type="checkbox" bind:checked={temp_settings.check_tauri_update}>
          <span class="slider"></span>
        </label>
      </div>
      <div class="settings-option">
        <span use:tippy={{ content: "the maximum amount of mods that can be downloaded simultaniously (set lower if you have a bad internet connection)" }} >max concurrent downloads</span>
        <input type="range" min="10" max="85" style="padding: 0; width: 5rem" bind:value={temp_settings.max_concurrent_downloads}>
        <span style="margin-right: 1rem;">{ temp_settings.max_concurrent_downloads }</span>
      </div>
      <div class="settings-option">
        <span use:tippy={{ content: "this will search for the newest versions of all mods instead of downloading the pre-set versions (not recommended and can cause issues)" }} >bleeding edge updates</span>
        <label class="switch">
          <input type="checkbox" bind:checked={temp_settings.bleeding_edge_updates}>
          <span class="slider"></span>
        </label>
      </div>
    </div>
  </div>
</main>

<style>
  #settings-modal {
    position: relative;
    width: 500px;
    height: 70%;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    background-color: rgba(0, 0, 0, 0.25);
    border-radius: 0.5rem;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  #close-button {
    position: absolute;
    top: 1rem;
    right: 1rem;
    fill: rgba(255, 255, 255, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
  }

  .settings-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: .8rem;
    padding-left: 1rem;
    width: 75%;
    height: 2.5rem;
    background-color: rgba(0, 0, 0, 0.1);
    border-radius: 0.5rem;
    font-size: .9rem;
  }
</style>
