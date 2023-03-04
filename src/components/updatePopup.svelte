<script>
    import { state } from "../stores/state";
    import { open } from '@tauri-apps/api/shell';
</script>

<main style="background-color: #2f2f2f;">
    <div class="info-div" style="vertical-align: middle; justify-content: center; display: flex; z-index: 999;">
        <!-- svelte-ignore a11y-missing-attribute -->
        <div class="info-box">
            <h1>update available!</h1>
            <a>{$state.updatePopup.notes}</a>
            <a>version: {$state.updatePopup.version}</a>
            <a>published: {$state.updatePopup.pub_date}</a>
            <span>
                <button on:click={() => open($state.updatePopup.url)} class="updateButton">update!</button>
                <button on:click={() => $state.updatePopup.shown = false} class="closeButton">close</button>
            </span>
        </div>
    </div>
</main>

<style>
.updateButton {
  width: 6rem;
  margin-top: 1em;
  background-color: #0d3b20;
}
.closeButton {
  width: 6rem;
  margin-top: 1em;
}
.closeButton:hover {
  border-color: rgb(109, 15, 3);
}

.info-div {
  animation: fadeIn 0.2s;
  transform: translateX(-50%) translateY(-50%);
  top: 50%;
  left: 50%;
  position: absolute;
  background-color: rgba(37, 37, 37, 0.5);
  width: 100%;
  height: 100%;
}

.info-box {
  transform: translateX(-50%) translateY(-50%);
  top: 50%;
  left: 50%;
  position: absolute;
  background-color: rgb(24, 24, 24);
  display: grid;
  padding: 1em;
  border-radius: 1.5em;
}
</style>