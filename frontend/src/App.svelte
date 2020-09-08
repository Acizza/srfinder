<script lang="ts">
  import RouteMap from "./components/RouteMap/index.svelte";
  import MainPanel from "./components/MainPanel/index.svelte";
  import type { Route, Airport } from "./components/MainPanel/RouteInfo/types";
  import { Theme, curTheme } from "./settings/theme";

  export let selectedRoute: Route | undefined = undefined;

  let routeMapRef: any = undefined;

  function viewAirport(event: CustomEvent<Airport>) {
    routeMapRef
      ?.viewAirport(event.detail)
      .catch((err: any) => console.error(err));
  }

  Theme.apply($curTheme);
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
    --bg-color: #353538;
  }

  :global(html[data-theme="dark"]) {
    --hover-color: #464649;
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

    --help-text-background-color: #302f2f;
    --help-text-color: #73758d;
  }

  :global(html[data-theme="light"]) {
    --hover-color: #fbfbff;
    --secondary-hover-color: #cacad1;
    --bg-color: #d3d3db;
    --tab-color: #ebebf1;
    --border-color: #a0a0a0;

    --text-color: #565656;
    --button-color: #e5e5eb;

    --error-color: #9e1010;
    --error-text-color: #e2cdcd;

    --disabled-text-color: #797979;
    --disabled-color: #c0c0c0;

    --scrollbar-main-color: #707070;
    --scrollbar-track-color: #acacac;

    --help-text-background-color: #949496;
    --help-text-color: #b4b6ce;
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
  <MainPanel bind:selectedRoute on:view-airport={(arpt) => viewAirport(arpt)} />
</main>
