<template>
  <form v-on:submit.prevent="submitted" class="filter-form">
    <speed-input
      :initial-speed="state.speed"
      @input="state.speed = $event"
      @has-error="setError('speed', $event)"
    />
    <airport-filters
      label="Departure"
      class="departure-filters"
      @changed="state.departure = $event"
      @has-error="setError('departure', $event)"
    />
    <airport-filters
      label="Arrival"
      class="arrival-filters"
      @changed="state.arrival = $event"
      @has-error="setError('arrival', $event)"
    />
    <input type="submit" class="find-routes-btn" value="Find Routes" :disabled="hasError" />
  </form>
</template>

<script lang="ts">
import { Component } from "vue-property-decorator";
import SpeedInput, { Speed, SpeedType } from "./SpeedInput.vue";
import AirportFilters, {
  State as AirportFilterState
} from "./AirportFilters/AirportFilters.vue";
import { VueWithErrorCatcher, ErrorState } from "../../../util/vue_with_error";

export interface State {
  speed: Speed;
  departure?: AirportFilterState;
  arrival?: AirportFilterState;
  timeDist?: any;
}

interface ErrorStates extends ErrorState {
  speed: boolean;
  departure: boolean;
  arrival: boolean;
  timeDist: boolean;
}

@Component({ components: { SpeedInput, AirportFilters } })
export default class FilterForm extends VueWithErrorCatcher<ErrorStates> {
  private state: State = {
    speed: new Speed("0.770", SpeedType.Mach)
  };

  constructor() {
    super({
      speed: false,
      departure: false,
      arrival: false,
      timeDist: false
    });
  }

  private submitted() {
    this.$emit("submitted", this.state);
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
