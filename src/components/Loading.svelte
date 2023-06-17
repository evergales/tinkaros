<script>
    import { fade } from "svelte/transition";
</script>

<div id="loading-wrapper">
  <div class="loading" transition:fade="{{duration: 200}}">
    <span></span>
    <span></span>
    <span></span>
    <span></span>
    <span></span>
  </div>
</div>

<style>
#loading-wrapper {
  position: absolute;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.loading span {
  display: inline-block;
  vertical-align: middle;
  width: .6em;
  height: .6em;
  margin: .19em;
  background: #5fdb9d;
  border-radius: .6em;
  animation: loading 1s infinite alternate;
}

.loading span:nth-of-type(2) {
  background: #4cce8d;
  animation-delay: 0.2s;
}
.loading span:nth-of-type(3) {
  background: #3ec280;
  animation-delay: 0.4s;
}
.loading span:nth-of-type(4) {
  background: #2bb16e;
  animation-delay: 0.6s;
}
.loading span:nth-of-type(5) {
  background: #1e8f56;
  animation-delay: .8s;
}

@keyframes loading {
  0% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}
</style>