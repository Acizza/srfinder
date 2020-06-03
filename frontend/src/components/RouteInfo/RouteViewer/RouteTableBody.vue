<template>
  <div class="route-table-container">
    <route-table />
    <div class="container-body">
      <slot />
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";
import RouteTable from "./RouteTable.vue";

@Component({ components: { RouteTable } })
export default class RouteTableBody extends Vue {}
</script>

<style scoped>
.route-table-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.container-body {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2.75vh;
}
</style>
