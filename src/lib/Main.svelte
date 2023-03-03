<script>
  import { config } from "../stores/config";
  import Launcher from "./Launcher.svelte";
  import Updater from "./Updater.svelte";
</script>
<main>
  {#if $config.launcher == "ahms"}
    <Launcher />
  {:else}
    <Updater />
  {/if}
</main>