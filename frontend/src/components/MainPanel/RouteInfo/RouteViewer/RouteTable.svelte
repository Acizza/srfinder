<script lang="ts">
  import type { Route, Airport } from "../types";
  import { createEventDispatcher } from "svelte";
  import {
    curDistanceUnit,
    DistanceUnit,
    DistanceUnitKind,
  } from "../../../../settings/units";

  export let routes: Route[] = [];
  export let selectedRoute: Route | undefined = undefined;

  let distUnitName: string;

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

  function distanceName(unit: DistanceUnitKind): string {
    switch (unit) {
      case DistanceUnitKind.NauticalMiles:
        return "NM";
      case DistanceUnitKind.Miles:
        return "MI";
      case DistanceUnitKind.Kilometers:
        return "KM";
    }
  }

  $: {
    distUnitName = distanceName($curDistanceUnit);
    // Since our unit changed, we need to redraw our routes
    routes = routes;
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
    cursor: default;
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
    cursor: default;
  }

  td.clickable {
    cursor: pointer;
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
    <th>{distUnitName}</th>
    <th>Time</th>
  </thead>
  <tbody>
    {#each routes as route}
      <tr
        on:mouseover={() => (selectedRoute = route)}
        class:selected={selectedRoute === route}>
        <td class="clickable" on:click={() => viewAirport(route.from)}>
          {route.from.icao}
        </td>
        <td class="clickable" on:click={() => viewAirport(route.to)}>
          {route.to.icao}
        </td>
        <td>{DistanceUnit.toCurrent(route.distance)}</td>
        <td>{routeTime(route)}</td>
      </tr>
    {/each}
  </tbody>
</table>
