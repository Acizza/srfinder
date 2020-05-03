<template>
  <div id="app">
    <RouteMap />
    <RouteInfo />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import RouteMap from "./components/RouteMap.vue";
import RouteInfo from "./components/RouteInfo.vue";

@Component({
  components: {
    RouteMap,
    RouteInfo
  }
})
export default class App extends Vue {}
</script>

<style>
:root {
  --hover-color: #555555;
  --bg-color: #444444;
  --tab-color: #333333;
  --border-color: #222222;
  --text-color: #d2d2d2;
}

body {
  background-color: var(--bg-color);
  color: var(--text-color);
  font-family: Lekton;
}

.uppercase {
  text-transform: uppercase;
}
</style>
