<script lang="ts">
  import { getContext } from "svelte";
  import { tabsContext } from "./index.svelte";
  import nextGlobalID from "../../id";

  const { registerTab, selected, renderNonSelected, canScroll } = getContext(
    tabsContext
  );
  const id = nextGlobalID();

  registerTab(id);

  $: isSelected = $selected === id;
  $: contentStyle = canScroll ? "overflow: hidden auto" : undefined;
</script>

<style>
  .tab-content {
    flex: 1 1 0;
  }

  .tab-content.hidden {
    display: none;
  }
</style>

<svelte:options immutable />

{#if renderNonSelected || isSelected}
  <div
    class="tab-content"
    class:scrollbar={canScroll}
    class:hidden={!isSelected}
    style={contentStyle}>
    <slot />
  </div>
{/if}
