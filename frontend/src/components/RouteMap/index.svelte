<script lang="ts">
  import Spinner from "../Spinner.svelte";
  import { onMount, onDestroy } from "svelte";
  import { setDefaultOptions, loadModules } from "esri-loader";
  import type { Airport, Route } from "../MainPanel/RouteInfo/types";
  import { ThemeKind, curTheme } from "../../settings/theme";
  import { AirportRunways } from "./runways";
  import type Map from "esri/Map";
  import type Basemap from "esri/Basemap";
  import type { Point, Polyline } from "esri/geometry";
  import type {
    SimpleLineSymbol,
    SimpleMarkerSymbol,
    TextSymbol,
  } from "esri/symbols";

  export let selectedRoute: Route | undefined = undefined;

  let mapContainer: any = undefined;

  let map: Map;
  let view: any;
  let airportRunways: AirportRunways;

  let drawingRoute = false;

  let textColor: string;
  let basemap: string;

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

    map = new Map({ basemap });
    view = new MapView({ container: mapContainer, map, zoom: 2 });

    const basemapToggle = new BasemapToggle({ view, nextBasemap: "hybrid" });
    view?.ui.add(basemapToggle, "bottom-right");

    airportRunways = await AirportRunways.initAsync(map);
  });

  onDestroy(() => {
    if (view) view.container = null;
  });

  export async function viewAirport(airport: Airport) {
    if (!view) return;

    const [Point] = await loadModules(["esri/geometry/Point"]);

    const center = new Point({
      latitude: airport.position.latitudeDeg,
      longitude: airport.position.longitudeDeg,
    });

    view.goTo({ center, scale: airportRunways.layer.minScale / 2 || 100_000 });
  }

  async function drawRouteAndRunways(route: Route | undefined) {
    if (!view || drawingRoute) return;

    view.graphics.removeAll();
    airportRunways.clear();

    if (!route) return;

    drawingRoute = true;

    try {
      await drawRoute(route);
      await airportRunways.draw(route.from, textColor);
      await airportRunways.draw(route.to, textColor);
    } catch (err) {
      throw err;
    } finally {
      drawingRoute = false;
    }
  }

  async function drawRoute(route: Route) {
    const [
      SimpleMarkerSymbol,
      SimpleLineSymbol,
      TextSymbol,
      Polyline,
      geometryEngine,
      Point,
      Graphic,
    ] = await loadModules([
      "esri/symbols/SimpleMarkerSymbol",
      "esri/symbols/SimpleLineSymbol",
      "esri/symbols/TextSymbol",
      "esri/geometry/Polyline",
      "esri/geometry/geometryEngine",
      "esri/geometry/Point",
      "esri/Graphic",
    ]);

    const pointFromArpt = (arpt: Airport): Point =>
      new Point({
        latitude: arpt.position.latitudeDeg,
        longitude: arpt.position.longitudeDeg,
      });

    const depPos = pointFromArpt(route.from);
    const arrPos = pointFromArpt(route.to);

    const linePath: Polyline = new Polyline({
      paths: [
        [depPos.x, depPos.y],
        [arrPos.x, arrPos.y],
      ],
    });

    const lineSymbol: SimpleLineSymbol = new SimpleLineSymbol({
      width: 2,
    });

    const geodesicLine: Polyline = geometryEngine.geodesicDensify(
      linePath,
      10_000
    );

    const symbol: SimpleMarkerSymbol = new SimpleMarkerSymbol({
      style: "diamond",
      size: "10px",
    });

    view.graphics.add(new Graphic(depPos, symbol));
    view.graphics.add(new Graphic(arrPos, symbol));
    view.graphics.add(new Graphic(geodesicLine, lineSymbol));

    let nameProps = {
      color: textColor,
      text: "DEP",
      yoffset: 7,
      font: { size: 8, family: "sans-serif" },
    };

    const depMarker: TextSymbol = new TextSymbol(nameProps);
    nameProps.text = "ARR";
    const arrMarker: TextSymbol = new TextSymbol(nameProps);

    view.graphics.add(new Graphic(depPos, depMarker));
    view.graphics.add(new Graphic(arrPos, arrMarker));
  }
</script>

<style>
  .route-map {
    display: flex;
    flex: 1 1 85%;
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
