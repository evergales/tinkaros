<script>
  import UpdatePopup from "../components/updatePopup.svelte";
  import { config } from "../stores/config";
  import { state } from "../stores/state";
  import Launcher from "./Launcher.svelte";
  import Updater from "./Updater.svelte";
</script>
<main>
  {#if $state.updatePopup.shown}
    <UpdatePopup />
  {/if}
  {#if $config.launcher == "ahms"}
    <Launcher />
  {:else}
    <Updater />
  {/if}
</main>