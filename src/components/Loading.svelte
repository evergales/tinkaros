<main style="background-color: #2f2f2f;">
    <div class="loading-div" style="vertical-align: middle; justify-content: center; display: flex;">
      <div class="loading_box">
        <div class="loading"></div>
        <h1 style="color: #9b9b9b;">loading</h1>
      </div>
    </div>
</main>

<style>
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

.loading_box {
  transform: translateX(-50%) translateY(-50%);
  top: 50%;
  left: 50%;
  position: absolute;
  background-color: rgb(24, 24, 24);
  display: flex;
  padding: 2em;
  border-radius: 1.5em;
}

.loading {
  display: flex;
  justify-content: center;
  padding-top: 1em;
  padding-right: 0.3em;
}

.loading::after {
  content: "";
  width: 20px;
  height: 20px;
  border: 6px solid #9b9b9b;
  border-top-color: #1c7841;
  border-radius: 50%;
  transform: rotate(0.16turn);
  animation: loading 1s ease infinite;
}

@keyframes loading {
  from {
    transform: rotate(0turn);
  }
  
  to {
    transform: rotate(1turn);
  }
}
</style>