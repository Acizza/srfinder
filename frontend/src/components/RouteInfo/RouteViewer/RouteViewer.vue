<template>
  <div class="route-table-wrapper">
    <route-table-body v-if="loading">
      <span class="uppercase">Loading</span>
    </route-table-body>
    <route-table-body v-else-if="routes.length === 0">
      <span class="uppercase">No routes found</span>
    </route-table-body>
    <route-table v-else :routes="routes" />
  </div>
</template>

<script lang="ts">
import { Component, Prop } from "vue-property-decorator";
import Vue from "vue";
import RouteTable from "./RouteTable.vue";
import RouteTableBody from "./RouteTableBody.vue";
import Route from "../../../types/route";

@Component({ components: { RouteTable, RouteTableBody } })
export default class RouteViewer extends Vue {
  @Prop({ default: true }) private loading!: boolean;
  @Prop({ default: () => [] }) private routes!: Route[];
}
</script>

<style>
:root {
  --route-table-height: 40%;
}
</style>

<style scoped>
.route-table-wrapper {
  width: 100%;
  height: var(--route-table-height);
  overflow: hidden;
  border-bottom: 2px solid var(--border-color);
}
</style>
