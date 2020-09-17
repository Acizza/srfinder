<script lang="ts">
  import RouteInfo from "./RouteInfo/index.svelte";
  import Footer from "./Footer.svelte";
  import Settings from "./Settings/index.svelte";
  import type { Route } from "./RouteInfo/types";

  export let selectedRoute: Route | undefined = undefined;

  const enum Panel {
    RouteInfo,
    Settings,
  }

  let visiblePanel = Panel.RouteInfo;

  function toggleSettings() {
    switch (visiblePanel) {
      case Panel.RouteInfo:
        visiblePanel = Panel.Settings;
        break;
      case Panel.Settings:
        visiblePanel = Panel.RouteInfo;
        break;
    }
  }
</script>

<style>
  .main-panel {
    display: flex;
    flex-direction: column;
    flex-basis: 16em;
    border-left: 3px solid var(--border-color);
    height: 100vh;
    overflow: hidden;
  }

  .route-info-wrapper {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .route-info-wrapper.hidden {
    display: none;
  }
</style>

<div class="main-panel">
  <div
    class="route-info-wrapper"
    class:hidden={visiblePanel !== Panel.RouteInfo}>
    <RouteInfo bind:selectedRoute on:view-airport />
  </div>
  {#if visiblePanel === Panel.Settings}
    <Settings />
  {/if}
  <Footer on:toggle-settings={toggleSettings} />
</div>
