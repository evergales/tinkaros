<script>
  import { state } from "../stores/state";
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
    <div class="outer">
        <div class="inner" style="width: {$progress * 100}%;"></div>
    </div>
</main>

<style>
    .inner, .outer {
        height: 0.6em;
        border-radius: 8px;
    }

    .outer {
        width: 10em;
        margin: 10px auto;
        background-color: transparent;
        border: 1px solid #1c7841;
    }

    .inner {
        width: 40%;
        background-color: #1c7841;
    }
</style>