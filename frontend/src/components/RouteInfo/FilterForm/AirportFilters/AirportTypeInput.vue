<template>
  <div>
    <label>Type</label>
    <select class="airport-type-select" @change="changed" v-bind="$attrs">
      <option selected value="unknown"></option>
      <option value="large_airport">Large</option>
      <option value="medium_airport">Medium</option>
      <option value="small_airport">Small</option>
      <option value="closed">Closed</option>
      <option value="heliport">Heliport</option>
      <option value="seaplane_base">Seaplane Base</option>
    </select>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component } from "vue-property-decorator";

@Component({ inheritAttrs: false })
export default class AirportTypeInput extends Vue {
  private changed(event: any) {
    const value: string | undefined = event?.target?.value;
    this.$emit("change", value !== "unknown" ? value : undefined);
  }
}
</script>

<style scoped>
.airport-type-select {
  width: 10vh;
}
</style>
