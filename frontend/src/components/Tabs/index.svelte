<script lang="ts" context="module">
  export const tabsContext = {};
</script>

<script lang="ts">
  import { setContext, onMount } from "svelte";
  import { writable } from "svelte/store";

  export let headers: string[] = [];

  let tabs: number[] = [];
  const selected: any = writable(null);

  setContext(tabsContext, {
    selected,
    registerTab(id: number) {
      tabs = [...tabs, id];
    },
  });

  function changeTab(index: number) {
    $selected = tabs[index];
  }

  onMount(() => {
    changeTab(0);
  });
</script>

<style>
  .tabs {
    display: flex;
    flex-direction: column;
    flex: 1 1 0;
  }

  .tab-headers {
    display: flex;
    justify-content: space-evenly;
    flex-wrap: nowrap;
    overflow: hidden;
    list-style: none;
    padding: 0;
    margin: 0 0 1em;
    font-size: 1.15em;
    border-bottom: 1px solid var(--border-color);
    cursor: pointer;
  }

  .tab-header {
    display: flex;
    padding: 0.75em 0;
    height: 100%;
    width: 100%;
    justify-content: center;
  }

  .tab-header:hover {
    background-color: var(--hover-color);
  }

  .tab-header:not(:last-child) {
    border-right: 1px solid var(--border-color);
  }
</style>

<div class="tabs">
  <ul class="tab-headers">
    {#each headers as header, idx}
      <li class="tab-header" on:click={() => changeTab(idx)}>{header}</li>
    {/each}
  </ul>
  <slot />
</div>
