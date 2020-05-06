<template>
  <form v-on:submit.prevent="onSubmitted" class="filter-form">
    <speed-input name="speed" @error="hasError = $event" />
    <input type="submit" class="find-routes-btn" value="Find Routes" :disabled="hasError" />
  </form>
</template>

<script lang="ts">
import { Component } from "vue-property-decorator";
import Vue from "vue";
import SpeedInput, { Speed } from "./SpeedInput.vue";

export interface State {
  speed: Speed;
  departure?: any;
  arrival?: any;
  timeDist?: any;
}

@Component({ components: { SpeedInput } })
export default class FilterForm extends Vue {
  private hasError = false;

  private onSubmitted(event: any) {
    const target = event.target;

    const state: State = {
      speed: Speed.parse(target.speed.value)
    };

    this.$emit("submitted", state);
  }
}
</script>

<style scoped>
.filter-form {
  display: flex;
  flex-direction: column;
  flex-wrap: nowrap;
  padding: 15px;
}

.speed-input {
  align-self: center;
}

.find-routes-btn {
  margin-top: 3vh;
  width: 60%;
  align-self: center;
  font-size: 3vh;
  padding: 5px;
}
</style>