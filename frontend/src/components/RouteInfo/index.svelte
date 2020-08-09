<script lang="ts">
  import Tabs from "../Tabs/index.svelte";
  import Tab from "../Tabs/Tab.svelte";

  export let routeSelected: ((route: any) => void) | null = null;
  export let airportClicked: ((airport: any) => void) | null = null;

  let loadingRoutes = false;
  let isFirstFetch = true;
  let routes: any[];
  let selectedRoute;

  const tabHeaders = ["FILTERS", "RUNWAYS", "FREQS"];
</script>

<svelte:options immutable />

<style>
  .route-info {
    display: flex;
    flex-direction: column;
    flex: 0 0 20%;
    border-left: 2px solid var(--border-color);
    height: 100vh;
  }
</style>

<div class="route-info">
  <Tabs headers={tabHeaders}>
    <Tab>
      <span>TODO (1)</span>
    </Tab>

    <Tab>
      <span>TODO (2)</span>
    </Tab>

    <Tab>
      <span>TODO (3)</span>
    </Tab>
  </Tabs>
</div>
