<script lang="ts">
  import Spinner from "../Spinner.svelte";
  import { onMount, onDestroy } from "svelte";
  import { setDefaultOptions, loadModules } from "esri-loader";
  import type { Airport, Route } from "../MainPanel/RouteInfo/types";
  import { ThemeKind, curTheme } from "../../settings/theme";
  import { AirportRunways } from "./runways";
  import type Map from "esri/Map";
  import type MapView from "esri/views/MapView";
  import type Basemap from "esri/Basemap";
  import type BasemapToggle from "esri/widgets/BasemapToggle";
  import { RouteLine } from "./route-line";
  import type { Point } from "esri/geometry";

  export let selectedRoute: Route | undefined = undefined;

  let mapContainer: any;

  let map: Map | undefined;
  let view: MapView | undefined;

  let routeLine: RouteLine | undefined;
  let airportRunways: AirportRunways | undefined;

  let textColor: string;
  let basemap: string;

  let drawingRoute = false;

  $: updateTheme($curTheme);
  $: drawRouteAndRunways(selectedRoute).catch((err) => console.error(err));

  async function updateTheme(theme: ThemeKind) {
    switch (theme) {
      case ThemeKind.Dark:
        textColor = "white";
        basemap = "dark-gray-vector";
        break;
      case ThemeKind.Light:
        textColor = "black";
        basemap = "gray-vector";
        break;
    }

    if (!map) return;

    map.basemap = (basemap as unknown) as Basemap;
    // Trigger a route redraw so it uses our new colors
    selectedRoute = selectedRoute;
  }

  onMount(async () => {
    // TODO: currently locked on version 4.14 because there's an issue in later versions where rotated text isn't properly aligned
    setDefaultOptions({ version: "4.14", css: true });

    const [Map, MapView, BasemapToggle] = await loadModules([
      "esri/Map",
      "esri/views/MapView",
      "esri/widgets/BasemapToggle",
    ]);

    // Casting tells TypeScript that these values won't be undefined anymore
    map = new Map({ basemap }) as Map;
    view = new MapView({ container: mapContainer, map, zoom: 2 }) as MapView;

    const basemapToggle: BasemapToggle = new BasemapToggle({
      view,
      nextBasemap: "hybrid",
    });

    view.ui.add(basemapToggle, "bottom-right");

    // Order matters here for drawing.
    // We want our route line to draw on top of airport runways.
    airportRunways = await AirportRunways.initAsync(map);
    routeLine = await RouteLine.initAsync(map);
  });

  onDestroy(() => {
    // Destroy the map view
    if (view) view.container = null as any;
  });

  export async function viewAirport(airport: Airport) {
    if (!view) return;

    const [Point] = await loadModules(["esri/geometry/Point"]);

    const center: Point = new Point({
      latitude: airport.position.latitudeDeg,
      longitude: airport.position.longitudeDeg,
    });

    // Set our scale to be at the edge of the runway layer
    const scale = airportRunways ? airportRunways.layer.minScale / 2 : 100_000;

    view.goTo({ center, scale });
  }

  async function drawRouteAndRunways(route: Route | undefined): Promise<void> {
    if (drawingRoute) return;

    routeLine?.clear();
    airportRunways?.clear();

    if (!route) return;

    drawingRoute = true;

    const result = Promise.all([
      routeLine?.draw(route, textColor),
      airportRunways?.draw(route.from, textColor),
      airportRunways?.draw(route.to, textColor),
    ]);

    return result.then(() => {}).finally(() => (drawingRoute = false));
  }
</script>

<style>
  .route-map {
    display: flex;
    flex: 1 1;
  }

  :global(.route-map .esri-view-surface::after) {
    outline: 0 !important;
  }

  .container.loaded {
    width: 100%;
    height: 100%;
  }
</style>

<div class="route-map">
  {#if !map}
    <Spinner />
  {/if}
  <div class="container" class:loaded={map} bind:this={mapContainer} />
</div>
