<script lang="ts">
  import Tabs from "../Tabs/index.svelte";
  import Tab from "../Tabs/Tab.svelte";
  import FilterForm from "./FilterForm/index.svelte";
  import type { FindRoutesQuery } from "./FilterForm/types";

  const tabHeaders = ["FILTERS", "RUNWAYS", "FREQS"];

  function findRoutes(query: CustomEvent<FindRoutesQuery>) {
    console.log(JSON.stringify(query.detail));
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
      <FilterForm on:findroutes={findRoutes} />
    </Tab>

    <Tab>
      <span>TODO (2)</span>
    </Tab>

    <Tab>
      <span>TODO (3)</span>
    </Tab>
  </Tabs>
</div>
