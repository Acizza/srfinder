<script lang="ts">
  import Spinner from "../../../Spinner.svelte";
  import type { Route } from "../types";
  import RouteTable from "./RouteTable.svelte";

  export let selectedRoute: Route | undefined = undefined;

  export let routes: Route[];
  export let loading: boolean;
  export let firstRender: boolean;
</script>

<style>
  .route-viewer.hidden {
    max-height: 0;
  }

  .route-viewer:not(.hidden) {
    border-bottom: 1px solid var(--border-color);
  }

  .route-viewer {
    display: flex;
    flex-direction: column;
    flex: 1 0 40%;
    max-height: 35vh;
    transition: max-height 0.3s ease-in-out;
    overflow: hidden auto;
  }

  .route-viewer .container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .no-routes-text {
    justify-content: center;
    align-self: center;
    font-size: 1.5em;
  }
</style>

<div class="route-viewer scrollbar" class:hidden={firstRender}>
  {#if loading || routes.length === 0}
    <RouteTable />
    <div class="container">
      {#if loading}
        <Spinner />
      {:else}<span class="no-routes-text">NO ROUTES FOUND</span>{/if}
    </div>
  {:else}
    <RouteTable {routes} bind:selectedRoute on:view-airport />
  {/if}
</div>
