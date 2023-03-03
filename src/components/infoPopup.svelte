<script>
  import { state } from "../stores/state";
</script>

<main style="background-color: #2f2f2f;">
    <div class="loading-div" style="vertical-align: middle; justify-content: center; display: flex;">
        <!-- svelte-ignore a11y-missing-attribute -->
        <div class="info-box">
            <h1>coming soon!</h1>
            <a>because of issues with authentication I was unable to create a custom launcher so far..</a>
            <a>later when I actually can spend money on this project I will :)</a>
            <button on:click={() => $state.infoPopupShown = false} class="closeButton">back</button>
        </div>
    </div>
</main>

<style>
.closeButton {
  width: 35%;
  margin-top: 1em;
  transform: translateX(90%);
  background-color: #0d3b20;
}

.loading-div {
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