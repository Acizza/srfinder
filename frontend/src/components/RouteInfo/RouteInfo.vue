<template>
  <div class="route-info">
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

@Component({ components: { Tabs, Tab, FilterForm } })
export default class RouteInfo extends Vue {
  private filtersSubmitted(state: FilterFormState) {
    console.log(JSON.stringify(state));
  }
}
</script>
