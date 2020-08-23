<script lang="ts">
  import { getContext } from "svelte";
  import { tabsContext } from "./index.svelte";
  import nextGlobalID from "../../id";

  const { registerTab, selected } = getContext(tabsContext);
  const id = nextGlobalID();

  registerTab(id);

  // Dormant tabs should be hidden
  $: tabStyle = ($selected !== id && "display: none") || undefined;
</script>

<style>
  .tab-content {
    flex: 1 1 0;
    overflow: hidden auto;
  }
</style>

<svelte:options immutable />

<div class="tab-content scrollbar" style={tabStyle}>
  <slot />
</div>
