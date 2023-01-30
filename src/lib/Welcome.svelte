<script lang="ts">
  import { state } from "../stores/state"
  import InfoPopup from "../components/infoPopup.svelte";
  import SelectCustomLauncher from "../components/selectCustomLauncher.svelte";
  let mainselect: boolean = true;

  let welcome_to: String = "launcher";
  let c: number = 0;
  async function changing_text() {    
    setInterval(() => {
      var texts  = [ "launcher", "installer", "updater", "manager" ];
      c++
      if (c == 4) {c = 0}
      welcome_to = texts[c]
    }, 4000)
  }
  changing_text()

</script>
<main>
  {#if mainselect}
    {#if $state.infoPopupShown}<InfoPopup />{/if}
    <div class="row">
        <h1>Welcome to AHMS&nbsp;</h1>
        {#key welcome_to}
        <h1 style="animation: fadeIn 500ms;">{welcome_to}!</h1>
        {/key}
      </div>
    
      <p>choose what to use as a launcher</p>
      <div class="row">
          <button on:click={() => {$state.infoPopupShown = true}}><img src="./icon.png" alt="ahms icon" height="100" width="100"></button>
          <h1 style="padding: 0.3em; padding-top: 0.65em;">or</h1>
          <button on:click={() => mainselect = false}><img src="./other.png" alt="ahms icon" height="100" width="100"></button>
    </div>
  {:else} <!-- subselect -->
    <SelectCustomLauncher />
  {/if}
</main>