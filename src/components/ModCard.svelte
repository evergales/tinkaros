<script lang="ts">
  import { open } from '@tauri-apps/api/shell';
  import { fade } from 'svelte/transition';
  export let any_mod: Mod;

  let curseforge_mod: CurseForgeMod = any_mod.CurseForgeMod;
  let modrinth_mod: ModrinthProject = any_mod.ModrinthProject;

  function openUrl() {
    open(modrinth_mod ? `https://modrinth.com/mod/${modrinth_mod.slug}` : `${curseforge_mod.links.websiteUrl}`)
  }

  function truncateDesctiption(str: string, maxLength: number) {
    if (str.length <= maxLength) {
      return str;
    }

    const truncated = str.slice(0, maxLength);
    const lastWhitespaceIndex = truncated.lastIndexOf(" ");

    if (lastWhitespaceIndex === -1) {
      return truncated + "...";
    } else {
      return truncated.slice(0, lastWhitespaceIndex) + "...";
    }
  }
</script>

<main>
    <div id="card" class="container" transition:fade="{{duration: 200}}" on:click={openUrl} on:keydown={openUrl}>
    {#if modrinth_mod}
      <img loading="lazy" src="./modrinth.png" alt="" id="platform-logo">
      <img loading="lazy" src="{modrinth_mod.icon_url}" alt="icon" id="card-icon">

      <div id="card-content">
        <p id="card-title">{modrinth_mod.title}</p>
        <p style="font-size: .6rem; margin: 0;">{truncateDesctiption(modrinth_mod.description, 70)}</p>
      </div>
    {:else if curseforge_mod}
      <img loading="lazy" src="./curseforge.png" alt="" id="platform-logo">
      <img loading="lazy" src="{curseforge_mod.logo?.url}" alt="icon" id="card-icon">

      <div id="card-content">
        <p id="card-title">{curseforge_mod.name}</p>
        <p style="font-size: .6rem; margin: 0;">{truncateDesctiption(curseforge_mod.summary, 70)}</p>
      </div>
    {/if}
    </div>
</main>

<style>
  #card {
    position: relative;
    height: 5rem;
    margin: .5rem;
    overflow: hidden;
    cursor: pointer;
  }

  #card-icon {
    position: absolute;
    height: 4rem;
    width: 4rem;
    object-fit: contain;
    margin: .5rem;
    border-radius: 5px;
  }

  #card-content {
    position: absolute;
    display: flex;
    flex-direction: column;
    margin-top: .75rem;
    margin-left: 5rem;
    margin-right: 2%;
    line-height: 1rem;
  }

  #card-title {
    font-size: .8rem;
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 12rem;
  }

  #platform-logo {
    z-index: 2;
    position: absolute;
    width: .8rem;
    height: .8rem;
    top: .5rem;
    left: .5rem;

    background-color: rgba(0, 0, 0, .8);
    padding: .2rem;
    border-radius: 4px;
  }
</style>