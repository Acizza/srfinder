<template>
  <div id="app">
    <RouteMap />
    <RouteInfo />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import RouteMap from "./components/RouteMap.vue";
import RouteInfo from "./components/RouteInfo/RouteInfo.vue";

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

input,
select {
  background-color: #3c3e42;
  border: 1px solid var(--border-color);
  color: var(--text-color);
  font-family: Lekton;
}

input {
  font-size: 2.5vh;
}

input:disabled {
  background-color: #303135;
  color: #666666;
}

input[type="submit"] {
  transition: background-color 200ms ease-in-out;
}

input[type="submit"]:hover:not(:disabled) {
  background-color: var(--hover-color);
}

select {
  font-size: 2vh;
}

.uppercase {
  text-transform: uppercase;
}

.route-map {
  width: 80%;
  height: 100%;
  position: fixed;
  left: 0;
  top: 0;
}

.route-info {
  width: 20%;
  height: 100%;
  position: fixed;
  top: 0;
  right: 0;
  border-left: 2px solid var(--border-color);
}
</style>
