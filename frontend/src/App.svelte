<script lang="ts">
  import RouteMap from "./components/RouteMap.svelte";
  import RouteInfo from "./components/RouteInfo/index.svelte";
  import type { Route, Airport } from "./components/RouteInfo/types";

  export let selectedRoute: Route | undefined = undefined;

  let routeMapRef: any = undefined;

  function viewAirport(event: CustomEvent<Airport>) {
    routeMapRef
      ?.viewAirport(event.detail)
      .catch((err: any) => console.error(err));
  }
</script>

<style>
  :global(.scrollbar) {
    scrollbar-width: thin;
    scrollbar-color: var(--scrollbar-main-color) var(--scrollbar-track-color);
  }

  :global(body) {
    background-color: var(--bg-color);
    color: var(--text-color);
    font-family: Lekton, sans-serif;
    margin: 0;
  }

  :global(:root) {
    --hover-color: #545458;
    --secondary-hover-color: #3e3e41;
    --bg-color: #353538;
    --tab-color: #2e2e30;
    --border-color: #222222;

    --text-color: #d2d2d2;
    --button-color: #323436;

    --error-color: #9e1010;
    --error-text-color: #e2cdcd;

    --disabled-text-color: #9999;
    --disabled-color: #2d2d31;

    --scrollbar-main-color: #525257;
    --scrollbar-track-color: #2d2d31;
  }

  :global(input, select) {
    background-color: var(--button-color);
    border: 1px solid var(--border-color);
    color: var(--text-color);
    font-family: Lekton;
  }

  :global(input) {
    font-size: 1em;
  }

  :global(input:invalid:focus) {
    outline: none;
  }

  :global(input[type="submit"]) {
    transition: background-color 200ms ease-in-out;
  }

  :global(input[type="submit"]:hover) {
    background-color: var(--hover-color);
  }

  main {
    width: 100%;
    height: 100%;
    display: flex;
  }
</style>

<main>
  <RouteMap {selectedRoute} bind:this={routeMapRef} />
  <RouteInfo bind:selectedRoute on:view-airport={(arpt) => viewAirport(arpt)} />
</main>
