<script lang="ts">
  import Spinner from "./Spinner.svelte";
  import { onMount, onDestroy } from "svelte";
  import { setDefaultOptions, loadModules } from "esri-loader";
  import type { Airport, Route } from "./MainPanel/RouteInfo/types";
  import { currentTheme, Theme } from "../theme";

  export let selectedRoute: Route | undefined = undefined;

  let mapContainer: any = undefined;

  let map: any;
  let view: any;
  let runwayLayer: any;

  let drawingRoute = false;

  let textColor: string;
  let basemap: string;

  $: updateTheme($currentTheme);
  $: drawRouteAndRunways(selectedRoute).catch((err) => console.error(err));

  function updateTheme(theme: Theme) {
    switch (theme) {
      case Theme.Dark:
        textColor = "white";
        basemap = "dark-gray-vector";
        break;
      case Theme.Light:
        textColor = "black";
        basemap = "gray-vector";
        break;
    }

    if (!map) return;

    map.basemap = basemap;
    // Trigger a route redraw so it uses our new colors
    selectedRoute = selectedRoute;
  }

  onMount(async () => {
    // TODO: currently locked on version 4.14 because there's an issue in later versions where rotated text isn't properly aligned
    setDefaultOptions({ version: "4.14", css: true });

    const [Map, MapView, BasemapToggle, GraphicsLayer] = await loadModules([
      "esri/Map",
      "esri/views/MapView",
      "esri/widgets/BasemapToggle",
      "esri/layers/GraphicsLayer",
    ]);

    map = new Map({ basemap });
    view = new MapView({ container: mapContainer, map, zoom: 2 });

    const basemapToggle = new BasemapToggle({ view, nextBasemap: "hybrid" });
    view?.ui.add(basemapToggle, "bottom-right");

    runwayLayer = new GraphicsLayer({ minScale: 200_000 });
    map?.add(runwayLayer);
  });

  onDestroy(() => {
    view.container = null;
  });

  export async function viewAirport(airport: Airport) {
    if (!view) return;

    const [Point] = await loadModules(["esri/geometry/Point"]);

    const center = new Point({
      latitude: airport.position.latitudeDeg,
      longitude: airport.position.longitudeDeg,
    });

    view.goTo({ center, scale: runwayLayer?.minScale / 2 || 100_000 });
  }

  async function drawRouteAndRunways(route: Route | undefined) {
    if (!view || drawingRoute) return;

    view.graphics.removeAll();

    if (!route) return;

    drawingRoute = true;

    try {
      await drawRoute(route);
      await drawAirportRunways(route.from);
      await drawAirportRunways(route.to);
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

    const pointFromArpt = (arpt: Airport) =>
      new Point({
        latitude: arpt.position.latitudeDeg,
        longitude: arpt.position.longitudeDeg,
      });

    const depPos = pointFromArpt(route.from);
    const arrPos = pointFromArpt(route.to);

    const linePath = new Polyline({
      paths: [
        [depPos.x, depPos.y],
        [arrPos.x, arrPos.y],
      ],
    });

    const lineSymbol = new SimpleLineSymbol({
      width: 2,
    });

    const geodesicLine = geometryEngine.geodesicDensify(linePath, 10_000);

    const marker = new SimpleMarkerSymbol({
      style: "diamond",
      size: "10px",
    });

    view.graphics.add(new Graphic(depPos, marker));
    view.graphics.add(new Graphic(arrPos, marker));
    view.graphics.add(new Graphic(geodesicLine, lineSymbol));

    let nameProps = {
      color: textColor,
      text: "DEP",
      yoffset: 7,
      font: { size: 8, family: "sans-serif" },
    };

    const depMarker = new TextSymbol(nameProps);
    nameProps.text = "ARR";
    const arrMarker = new TextSymbol(nameProps);

    view.graphics.add(new Graphic(depPos, depMarker));
    view.graphics.add(new Graphic(arrPos, arrMarker));
  }

  async function drawAirportRunways(airport: Airport) {
    if (airport.runways.length === 0 || !runwayLayer) return;

    const [
      SimpleLineSymbol,
      Point,
      Polyline,
      TextSymbol,
      Graphic,
    ] = await loadModules([
      "esri/symbols/SimpleLineSymbol",
      "esri/geometry/Point",
      "esri/geometry/Polyline",
      "esri/symbols/TextSymbol",
      "esri/Graphic",
    ]);

    const runwaySymbol = new SimpleLineSymbol({ width: 3, color: "black" });

    for (const runway of airport.runways) {
      if (!runway.heMarker || !runway.leMarker) continue;

      const hePos = new Point({
        x: runway.heMarker.position.longitudeDeg,
        y: runway.heMarker.position.latitudeDeg,
      });

      const lePos = new Point({
        x: runway.leMarker.position.longitudeDeg,
        y: runway.leMarker.position.latitudeDeg,
      });

      const runwayLine = new Polyline({
        paths: [
          [hePos.x, hePos.y],
          [lePos.x, lePos.y],
        ],
      });

      runwayLayer.graphics.add(new Graphic(runwayLine, runwaySymbol));

      let nameProps = {
        color: textColor,
        text: runway.heMarker.name,
        angle: angleFromPoints(hePos, lePos),
        yoffset: -10,
        font: { size: 8, family: "sans-serif" },
      };

      const heText = new TextSymbol(nameProps);
      runwayLayer.graphics.add(new Graphic(hePos, heText));

      nameProps.text = runway.leMarker.name;
      nameProps.angle += 180;

      const leText = new TextSymbol(nameProps);
      runwayLayer.graphics.add(new Graphic(lePos, leText));
    }
  }

  type Point = { x: number; y: number };

  // https://stackoverflow.com/a/18738281
  function angleFromPoints(start: Point, end: Point): number {
    const startRad: Point = {
      x: toRad(start.x),
      y: toRad(start.y),
    };

    const endRad: Point = {
      x: toRad(end.x),
      y: toRad(end.y),
    };

    const deltaLon = endRad.x - startRad.x;

    const y = Math.sin(deltaLon) * Math.cos(endRad.y);
    const x =
      Math.cos(startRad.y) * Math.sin(endRad.y) -
      Math.sin(startRad.y) * Math.cos(endRad.y) * Math.cos(deltaLon);

    return (toDeg(Math.atan2(y, x)) + 360) % 360;
  }

  function toRad(ang: number): number {
    return ang * (Math.PI / 180);
  }

  function toDeg(ang: number): number {
    return ang * (180 / Math.PI);
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
