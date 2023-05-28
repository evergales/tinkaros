<script>
  import { state } from "../stores/state";
  import { fade } from "svelte/transition"
  import { tweened } from "svelte/motion"
  import { cubicOut } from "svelte/easing"

  const progress = tweened(0, {
    duration: 500,
    easing: cubicOut
  })

  state.subscribe((prev_val) => {
    progress.set(prev_val.progress / 100)
  })

</script>

<main>
  <div class="outer" transition:fade="{{duration: 200}}">
    <div class="inner" style="width: {$progress * 100}%;"></div>
    {#if $state.progress > 0}
    <div class="progress-percentage">{$state.progress}%</div>
    {/if}
  </div>
</main>

<style>
.inner, .outer {
  height: 0.4rem;
  border-radius: 5px;
}

.outer {
  position: absolute;
  bottom: 5%;
  left: 5%;
  right: 5%;
  border: 1px solid rgba(37, 148, 81, 0.3);
}

.inner {
  width: 0%;
  background-color: rgba(37, 148, 81, 0.3);
}

.progress-percentage {
  position: absolute;
  top: -1.5rem;
  left: 50%;
  transform: translate(-50%, 0);
  font-size: 0.5rem;
  color: rgba(255, 255, 255, 0.5);
}
</style>