<script lang="ts">
  import type { Route, Airport } from "../types";
  import { createEventDispatcher } from "svelte";

  export let routes: Route[] = [];
  export let selectedRoute: Route | undefined = undefined;

  const dispatch = createEventDispatcher();

  function zeroPad(value: number): string {
    return value < 10 ? `0${value}` : value.toString();
  }

  function routeTime(route: Route): string {
    return `${zeroPad(route.time.hour)}:${zeroPad(route.time.minutes)}`;
  }

  function viewAirport(arpt: Airport) {
    dispatch("view-airport", arpt);
  }
</script>

<style>
  table {
    width: 100%;
    border-spacing: 0;
  }

  thead {
    position: sticky;
    top: 0;
    background-color: var(--tab-color);
  }

  th {
    padding: 0.5em;
    font-weight: normal;
    border-bottom: 1px solid var(--border-color);
  }

  th:not(:last-child),
  td:not(:last-child) {
    border-right: 1px solid var(--border-color);
  }

  td {
    padding: 0.5em;
    text-align: center;
    border-bottom: 1px solid var(--border-color);
  }

  tr.selected {
    background-color: var(--secondary-hover-color);
  }

  tr:hover {
    background-color: var(--secondary-hover-color);
  }

  td:hover {
    background-color: var(--hover-color);
  }
</style>

<table class="route-viewer">
  <thead>
    <th>From</th>
    <th>To</th>
    <th>NM</th>
    <th>Time</th>
  </thead>
  <tbody>
    {#each routes as route}
      <tr
        on:mouseover={() => (selectedRoute = route)}
        class:selected={selectedRoute === route}>
        <td on:click={() => viewAirport(route.from)}>{route.from.icao}</td>
        <td on:click={() => viewAirport(route.to)}>{route.to.icao}</td>
        <td>{Math.round(route.distance)}</td>
        <td>{routeTime(route)}</td>
      </tr>
    {/each}
  </tbody>
</table>
