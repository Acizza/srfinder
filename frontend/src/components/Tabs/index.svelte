<script lang="ts" context="module">
  export const tabsContext = {};
</script>

<script lang="ts">
  import { setContext, onMount } from "svelte";
  import { writable } from "svelte/store";

  export let headers: string[] = [];
  export let canScroll: boolean = true;

  let tabs: number[] = [];
  const selected: any = writable(null);

  setContext(tabsContext, {
    selected,
    canScroll,
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
    margin: 0;
    font-size: 1em;
    cursor: pointer;
    background-color: var(--tab-color);
  }

  .tab-header {
    display: flex;
    padding: 0.75em 0.5em;
    width: 100%;
    justify-content: center;
  }

  .tab-header:hover {
    background-color: var(--hover-color);
  }

  .tab-header.selected {
    background-color: var(--bg-color);
  }

  .tab-header:not(.selected) {
    border-bottom: 1px solid var(--border-color);
  }

  .tab-header:not(:last-child) {
    border-right: 1px solid var(--border-color);
  }
</style>

<div class="tabs">
  <ul class="tab-headers">
    {#each headers as header, idx}
      <li
        class="tab-header"
        class:selected={tabs[idx] === $selected}
        on:click={() => changeTab(idx)}>
        {header}
      </li>
    {/each}
  </ul>
  <slot />
</div>
