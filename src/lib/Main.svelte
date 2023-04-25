<script>
  import UpdatePopup from "../components/updatePopup.svelte";
  import { state } from "../stores/state";
  import Updater from "./Updater.svelte";
</script>
<main>
  {#if $state.updatePopup.shown}
    <UpdatePopup />
  {/if}
  <Updater />
</main>