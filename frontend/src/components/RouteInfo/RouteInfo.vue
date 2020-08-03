<template>
  <div class="route-info">
    <transition name="open-route-viewer">
      <route-viewer v-if="hasRequestedRoutes" :routes="routes" :loading="loadingRoutes" />
    </transition>
    <tabs>
      <tab name="Filters" :selected="true">
        <filter-form v-on:submitted="filtersSubmitted" />
      </tab>
      <tab name="Runways">
        <p>TODO (2)</p>
      </tab>
      <tab name="Freqs">
        <p>TODO (3)</p>
      </tab>
    </tabs>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import Tabs from "./Tabs/Tabs.vue";
import Tab from "./Tabs/Tab.vue";
import FilterForm, {
  State as FilterFormState,
} from "./FilterForm/FilterForm.vue";
import RouteViewer from "./RouteViewer/RouteViewer.vue";
import Route from "../../types/route";

@Component({ components: { Tabs, Tab, FilterForm, RouteViewer } })
export default class RouteInfo extends Vue {
  private routes: Route[] = [];
  private loadingRoutes = false;
  private hasRequestedRoutes = false;

  private filtersSubmitted(state: FilterFormState) {
    console.log(JSON.stringify(state));

    this.loadingRoutes = true;
    this.hasRequestedRoutes = true;

    this.findRoutes(state)
      .then((routes) => {
        this.routes = routes;
        this.loadingRoutes = false;
      })
      .catch((err) => {
        console.error(err);
        this.routes = [];
        this.loadingRoutes = false;
      });
  }

  private async findRoutes(state: FilterFormState): Promise<Route[]> {
    const resp = await fetch("/api/search_routes", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(state),
    });

    if (!resp.ok) {
      throw Error(
        `Searching for routes failed with response code: ${resp.status}`
      );
    }

    const json = await resp.json();

    if (!json.routes)
      throw Error(`Malformed json response while fetching routes: ${json}`);

    return json.routes;
  }
}
</script>

<style scoped>
.tabs {
  min-height: calc(100% - var(--route-table-height));
  overflow: hidden;
}

.open-route-viewer-enter-active,
.open-route-viewer-active {
  transition: height 250ms;
}

.open-route-viewer-enter,
.open-route-viewer-leave-to {
  height: 0%;
}
</style>
