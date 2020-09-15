<script lang="ts">
  import type { Route, Airport } from "../types";
  import { createEventDispatcher } from "svelte";
  import {
    curDistanceUnit,
    DistanceUnit,
    DistanceUnitKind,
  } from "../../../../settings/units";
  import SortableTable from "./SortableTable.svelte";

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

  const enum HeaderKind {
    From,
    To,
    Distance,
    Time,
  }

  $: headers = [
    { name: "From", index: HeaderKind.From },
    { name: "To", index: HeaderKind.To },
    { name: distUnitName, index: HeaderKind.Distance },
    { name: "Time", index: HeaderKind.Time },
  ];

  function sortComparer(headerIndex: number): (a: Route, b: Route) => number {
    function cmpBy<T, V>(key: (value: T) => V): (a: T, b: T) => number {
      return (a: T, b: T) => {
        const first = key(a);
        const second = key(b);

        if (first < second) return -1;
        if (first > second) return 1;

        return 0;
      };
    }

    let cmp: (a: Route, b: Route) => number;

    switch (headerIndex as HeaderKind) {
      case HeaderKind.From:
        cmp = cmpBy((value) => value.from.icao);
        break;
      case HeaderKind.To:
        cmp = cmpBy((value) => value.to.icao);
        break;
      case HeaderKind.Distance:
        cmp = (a, b) => a.distance - b.distance;
        break;
      case HeaderKind.Time:
        cmp = (a, b) => {
          const [fst, snd] = [a.time, b.time];
          return fst.hour * 60 - snd.hour * 60 + (fst.minutes - snd.minutes);
        };

        break;
    }

    return cmp;
  }
</script>

<style>
  :global(.route-viewer-table) {
    width: 100%;
  }

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

  tr.selected,
  tr:hover {
    background-color: var(--secondary-hover-color);
  }

  td:hover {
    background-color: var(--hover-color);
  }
</style>

<SortableTable
  {headers}
  className="route-viewer-table"
  comparer={sortComparer}
  data={routes}
  let:sortedData>
  {#each sortedData as route}
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
</SortableTable>
