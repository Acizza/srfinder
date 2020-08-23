<script lang="ts">
  import Tabs from "../Tabs/index.svelte";
  import Tab from "../Tabs/Tab.svelte";
  import FilterForm from "./FilterForm/index.svelte";
  import RouteViewer from "./RouteViewer/index.svelte";
  import type { FindRoutesQuery, Route } from "./types";

  const tabHeaders = ["FILTERS", "RUNWAYS", "FREQS"];

  let error: string | null = null;
  let loadingRoutes = false;
  let firstRouteFetch = true;
  let routes: Route[] = [];

  function routesRequested(query: CustomEvent<FindRoutesQuery>) {
    loadingRoutes = true;

    findRoutes(query.detail)
      .then((newRoutes) => {
        routes = newRoutes;
        firstRouteFetch = false;
        error = null;
      })
      .catch((err: Error) => {
        console.error(err);
        error = err.message;
      })
      .finally(() => {
        loadingRoutes = false;
      });
  }

  async function findRoutes(query: FindRoutesQuery): Promise<Route[]> {
    const resp = await fetch("/api/search_routes", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(query),
    });

    if (!resp.ok) throw Error(`Failed to find routes: ${resp.status}`);

    const json = await resp.json();

    if (!json.routes) throw Error("Received malformed json response");

    return json.routes;
  }
</script>

<style>
  .route-info {
    display: flex;
    flex-direction: column;
    flex: 1 0 15%;
    border-left: 2px solid var(--border-color);
    height: 100vh;
    overflow: hidden;
  }
</style>

<svelte:options immutable />
<div class="route-info">
  <RouteViewer {routes} loading={loadingRoutes} firstRender={firstRouteFetch} />
  <Tabs headers={tabHeaders}>
    <Tab>
      <FilterForm on:findroutes={routesRequested} {error} {loadingRoutes} />
    </Tab>

    <Tab>
      <span>TODO (2)</span>
    </Tab>

    <Tab>
      <span>TODO (3)</span>
    </Tab>
  </Tabs>
</div>
