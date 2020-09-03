<script lang="ts">
  import type { Route } from "../types";
  import AirportData from "./AirportData.svelte";

  export let route: Route | undefined;
</script>

<style>
  .airport-info {
    justify-content: center;
    display: flex;
    flex-direction: column;
    flex: 1 1;
  }

  .airport-info.display-message {
    height: 100%;
  }

  .airport-info .help-text {
    align-self: center;
    font-size: 1.4em;
    padding: 0.25em;
    cursor: default;
  }
</style>

<div class="airport-info" class:display-message={!route}>
  {#if route}
    <AirportData name={route.from.icao} airport={route.from} />
    <AirportData name={route.to.icao} airport={route.to} />
  {:else}<span class="help-text">SELECT AN AIRPORT</span>{/if}
</div>
