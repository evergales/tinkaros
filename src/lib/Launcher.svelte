<script async lang="ts">
    import { state } from "../stores/state";
    import Account from "../components/Account.svelte";
    async function start(
      e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
    ) {
      let btn = e.currentTarget;
      if (btn.disabled) return;
      btn.disabled = true;
      //await invoke("")
      setTimeout(() => {
        btn.disabled = false;
      }, 5000);
    }
  
    async function account() {
      $state.accountShown = !$state.accountShown
    }
  </script>
  
  <!-- svelte-ignore a11y-missing-attribute -->
  <main>
    {#if $state.accountShown}<Account />{/if}
    <button class="account" on:click={account}>
      <!-- player icon -->
      <div style="display: grid;">
        <a style="font-size: 0.8em">playername</a>
        <a style="font-size: 0.6em; opacity: 0.8;">logged in: </a>
        <!-- microsoft/mojang -->
      </div>
    </button>
    <div class="subbar">
      <button class="start" on:click={(e) => start(e)}
        ><a style="font-size: larger;">start</a></button
      >
      <div class="subbar-name">
        <!-- display "not logged in" if not logged in -->
        <!-- player icon -->
        <a>playername</a>
      </div>
    </div>
    <h1>still need to make a good looking ui</h1>
  </main>
  
  <style>
    @font-face {
      font-family: "Minecraft";
      src: url("/minecraft-bold.otf") format("opentype");
    }
    :root {
      font-family: Minecraft;
    }
  
    .account {
      position: fixed;
      display: flex;
      left: 0.5em;
      top: 0.5em;
      text-align: left;
    }
  
    .subbar {
      position: fixed;
      display: flex;
      justify-content: center;
      width: 100%;
      left: 0;
      right: 0;
      height: 5em;
      bottom: 0%;
      background-color: rgb(26, 26, 26);
    }
  
    .subbar-name {
      position: absolute;
      right: 1em;
      bottom: 35%;
      display: flex;
    }
  
    .start {
      transform: translateY(-25%);
      width: 12em;
      height: 4em;
      background-color: #0d3b20;
      transition: 0.2s;
    }
    .start:hover {
      border-color: #14723b;
      transform: translateY(-30%);
    }
  
    .start:disabled {
      background-color: rgb(59, 59, 59);
      cursor: default;
    }
  
    .start:disabled:hover {
      border-color: rgb(59, 59, 59);
      transform: translateY(-25%);
    }
  </style>
  