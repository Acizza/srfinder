<script lang="ts">
  import Tabs from "../Tabs/index.svelte";
  import Tab from "../Tabs/Tab.svelte";
  import FilterForm from "./FilterForm/index.svelte";
  import type { FindRoutesQuery, Route } from "./types";

  const tabHeaders = ["FILTERS", "RUNWAYS", "FREQS"];

  let error: string | null = null;
  let isLoadingRoutes = false;

  function routesRequested(query: CustomEvent<FindRoutesQuery>) {
    isLoadingRoutes = true;

    findRoutes(query.detail)
      .then((routes) => {
        console.log(routes);
        error = null;
      })
      .catch((err: Error) => {
        console.error(err);
        error = err.message;
      })
      .finally(() => {
        isLoadingRoutes = false;
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
  }
</style>

<svelte:options immutable />
<div class="route-info">
  <Tabs headers={tabHeaders}>
    <Tab>
      <FilterForm on:findroutes={routesRequested} {error} {isLoadingRoutes} />
    </Tab>

    <Tab>
      <span>TODO (2)</span>
    </Tab>

    <Tab>
      <span>TODO (3)</span>
    </Tab>
  </Tabs>
</div>
