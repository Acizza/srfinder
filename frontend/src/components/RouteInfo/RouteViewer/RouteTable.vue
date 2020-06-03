<template>
  <table class="route-table">
    <thead>
      <tr>
        <th>From</th>
        <th>To</th>
        <th>NM</th>
        <th>Time</th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(route, i) in routes"
        :key="i"
        @click="onClick(route)"
        @mouseover="onHovered(route)"
        :class="{ 'hovered-route': hovered === route }"
      >
        <td @click="airportClicked(route)">{{ route.from.icao }}</td>
        <td @click="airportClicked(route)">{{ route.to.icao }}</td>
        <td>{{ Math.round(route.distance) }}</td>
        <td>{{ routeTime(route) }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script lang="ts">
import { Component, Prop } from "vue-property-decorator";
import Vue from "vue";
import Route from "../../../types/route";
import Airport from "../../../types/airport";

function zeroPad(value: number): string {
  return value < 10 ? `0${value}` : value.toString();
}

@Component
export default class RouteTable extends Vue {
  @Prop({ default: () => [] }) private routes!: Route[];
  private hovered: Route | null = null;

  private routeTime(route: Route): string {
    return `${zeroPad(route.time.hour)}:${zeroPad(route.time.minutes)}`;
  }

  private airportClicked(airport: Airport) {
    this.$emit("airport-clicked", airport);
  }

  private onClick(route: Route) {
    this.$emit("clicked", route);
  }

  private onHovered(route: Route) {
    this.hovered = route;
    this.$emit("hovered", route);
  }
}
</script>

<style>
:root {
  --route-table-header-height: 4vh;
}
</style>

<style scoped>
.route-table {
  cursor: default;
  text-transform: uppercase;
  text-align: center;
  border-collapse: separate;
  border-spacing: 0;
  font-size: 2.25vh;
  width: 100%;
  overflow: hidden;
}

/*
    We want the table header to be sticky so it stays visible while scrolling.
    Due to the fun limitations of tables, there isn't really any clean way of making only
    the table body scrollable.
 */
.route-table thead {
  position: sticky;
  top: 0;
  background-color: var(--tab-color);
  height: var(--route-table-header-height);
}

.route-table tbody tr {
  transition: background-color 100ms ease-in-out;
}

.route-table tbody .hovered-route {
  background-color: var(--hover-color);
}

.route-table tr:first-child,
.route-table th,
.route-table td {
  border-bottom: 1px solid var(--border-color);
}

.route-table th,
.route-table td {
  font-weight: normal;
  padding-top: 0.75vh;
  padding-bottom: 0.75vh;
}

.route-table th:not(:first-child),
.route-table td:not(:first-child) {
  border-left: 1px solid var(--border-color);
}

/* Since our scrollbar is attached to the entire table, we need to manually move it below the header. */
.route-table-scrollbar .ps__rail-y {
  margin-top: var(--route-table-header-height) !important;
}
</style>
