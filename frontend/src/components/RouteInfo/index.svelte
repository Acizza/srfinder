<script lang="ts">
  import Tabs from "../Tabs/index.svelte";
  import Tab from "../Tabs/Tab.svelte";
  import FilterForm from "./FilterForm/index.svelte";

  const tabHeaders = ["FILTERS", "RUNWAYS", "FREQS"];
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
      <FilterForm />
    </Tab>

    <Tab>
      <span>TODO (2)</span>
    </Tab>

    <Tab>
      <span>TODO (3)</span>
    </Tab>
  </Tabs>
</div>
