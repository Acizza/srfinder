<script lang="ts">
  import type { Route } from "./types";
  import Spinner from "../Spinner.svelte";

  export let routes: Route[];
  export let loading: boolean;
  export let firstRender: boolean;

  function zeroPad(value: number): string {
    return value < 10 ? `0${value}` : value.toString();
  }

  function routeTime(route: Route): string {
    return `${zeroPad(route.time.hour)}:${zeroPad(route.time.minutes)}`;
  }
</script>

<style>
  .route-viewer.hidden {
    max-height: 0;
  }

  .route-viewer {
    display: flex;
    flex-direction: column;
    flex: 1 0 40%;
    max-height: 40%;
    transition: max-height 0.3s ease-in-out;
    border-bottom: 1px solid var(--border-color);
  }

  .headers {
    display: flex;
    flex-wrap: nowrap;
    justify-content: space-evenly;
    text-align: center;
  }

  .headers span {
    width: 100%;
    padding: 0.5em;
    background-color: var(--tab-color);
    border-bottom: 1px solid var(--border-color);
  }

  .headers span:not(:last-child),
  .route span:not(:last-child) {
    border-right: 1px solid var(--border-color);
  }

  .body {
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: center;
    flex: 1 0;
    overflow: hidden;
  }

  .route {
    display: flex;
    flex-wrap: nowrap;
    justify-content: space-evenly;
    text-align: center;
    border-bottom: 1px solid var(--border-color);
  }

  .route:hover {
    background-color: var(--hover-color);
  }

  .route span {
    padding: 0.5em;
    overflow: hidden;
    flex: 1 1 25%;
    max-width: 25%;
    min-width: 0;
  }

  .route span:hover {
    background-color: var(--secondary-hover-color);
  }
</style>

<div class="route-viewer" class:hidden={firstRender}>
  <div class="headers">
    <span>From</span>
    <span>To</span>
    <span>NM</span>
    <span>Time</span>
  </div>
  <div class="body">
    {#if loading}
      <Spinner />
    {:else}
      {#each routes as route}
        <div class="route">
          <span>{route.from.icao}</span>
          <span>{route.to.icao}</span>
          <span>{Math.round(route.distance)}</span>
          <span>{routeTime(route)}</span>
        </div>
      {/each}
    {/if}
  </div>
</div>
