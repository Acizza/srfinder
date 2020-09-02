<script lang="ts">
  import RouteInfo from "./RouteInfo/index.svelte";
  import type { Route } from "./RouteInfo/types";

  export let selectedRoute: Route | undefined = undefined;
</script>

<style>
  .main-panel {
    display: flex;
    flex-direction: column;
    flex: 1 0 15em;
    border-left: 3px solid var(--border-color);
    height: 100vh;
    overflow: hidden;
  }
</style>

<div class="main-panel">
  <RouteInfo bind:selectedRoute on:view-airport />
</div>
