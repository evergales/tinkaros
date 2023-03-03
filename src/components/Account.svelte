<script lang="ts">
  //import { invoke } from "@tauri-apps/api/tauri";
  import { state } from "../stores/state";
  let selected: string = "microsoft";
  let username: string;
  let email: string;
  let password: string;

  async function login(e: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement; }) {
    e.currentTarget.disabled = true 
    //console.log(await invoke("login", {selected, username, email, password}));
  }

  async function switchType(e: MouseEvent & { currentTarget: EventTarget & SVGSVGElement; }) {
    let clicked = e.currentTarget
    if (selected == clicked.id) return;
    let all = ["microsoft", "mojang"]
    let i= all.findIndex((v) => v == clicked.id);
    all.splice(i, 1)
    clicked.style.fill = "#1c7841"
    all.forEach((v) => {
        let e = document.getElementById(v)
        e.style.fill = "white"
    })
    selected = clicked.id
  }

</script>

<main>
    <!-- svelte-ignore a11y-missing-attribute -->
    <div class="accountPopup">
        {#if $state.loggedIn}
            <a>you are logged in as playername</a>
            <button>log out</button>
        {:else}
            <a>log in:</a>
            <a style="font-size: xx-small; opacity: 80%; position: fixed; transform: translateY(1.5em)">with {selected}</a>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="loginOptions">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" id="microsoft" style="fill: #1c7841;" on:click={(e) => {switchType(e)}}><path d="M0 32h214.6v214.6H0V32zm233.4 0H448v214.6H233.4V32zM0 265.4h214.6V480H0V265.4zm233.4 0H448V480H233.4V265.4z"/></svg>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" id="mojang" on:click={(e) => {switchType(e)}}><path d="M9.213 0c-5.088 0-9.213 4.125-9.213 9.213v22.787h22.787c5.088 0 9.213-4.125 9.213-9.213v-22.787zM22.697 3.459c0.756 0 1.365 3.339 1.365 4.093 0.025 0.771-0.593 1.407-1.365 1.407-0.771 0-1.391-0.636-1.364-1.407 0-0.755 0.615-4.093 1.364-4.093zM19.213 7.328c0.923 0.256 2.776 5.032 4.849 2.959 1.932-1.932 2.735 9.552 2.735 9.552l-2.735-1.365c0 0 0-2.728-4.093-5.457-5.079-3.391-10.923-1.156-10.923 4.088 0 10.713 17.751 6.828 17.751 6.828s-0.005 2.729-2.735 2.729h-16.375c-2.729 0-2.735-2.729-2.735-2.729v-13.645c0-2.735 2.735-2.735 2.735-2.735h5.457c2.729 0 5.459 2.735 5.459 2.735 0-1.989 0.156-2.803 0.437-2.943 0.057-0.027 0.115-0.032 0.172-0.016z"/></svg>
            </div>
            {#if selected == "microsoft" || selected == "mojang"}
                <input type="text" placeholder="email" bind:value={email}>
                <input type="password" placeholder="password" bind:value={password}>
            {:else if selected == "mojang"}
                <input type="text" placeholder="username" bind:value={username}>
                <input type="password" placeholder="password" bind:value={password}>
            {/if} <!-- im bad at coding shut up -->
            {#if selected == "microsoft" && email && password || selected == "mojang" && email && password} 
                <button class="loginButton" on:click={(e) => login(e)}>log in</button>
            {/if}
        {/if}
    </div>
</main>

<style>
    .accountPopup {
        position: absolute;
        display: grid;
        text-align: left;
        top: 1em;
        left: 8em;
        background-color: black;
        border-radius: 8px;
        background-color: rgb(26, 26, 26);
        padding: 1em;
        gap: 0.5em;
        transition: 0.5s;
    }
    .loginOptions {
        display: flex;
        position: fixed;
        gap: 0.3em;
        transform: translateX(350%);
    }
    .loginOptions svg {
        width: 1.5em;
        height: 1.5em;
        fill: white;
        cursor: pointer;
    }
    .loginButton {
        transform: translateX(50%);
        width: 50%;
        background-color: #0d3b20;
        animation: fadeIn 0.2s;
    }
    .loginButton:disabled {
        background-color: rgb(59, 59, 59);
        cursor: default;
    }
    .loginButton:disabled:hover {
        border-color: rgb(59, 59, 59);
    }

</style>