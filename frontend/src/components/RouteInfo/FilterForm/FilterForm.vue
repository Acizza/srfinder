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
    <time-dist-tabs
      @changed="state.timeDist = $event"
      @has-error="setError('timeDist', $event)"
    />
    <input
      type="submit"
      class="find-routes-btn"
      value="Find Routes"
      :disabled="hasError"
    />
  </form>
</template>

<script lang="ts">
import { Component } from "vue-property-decorator";
import { VueWithErrorCatcher, ErrorState } from "../../../util/vue_with_error";
import SpeedInput, { Speed, SpeedType } from "./SpeedInput.vue";
import AirportFilters, {
  State as AirportFilterState,
} from "./AirportFilters/AirportFilters.vue";
import TimeDistTabs, {
  Serialized as TimeDistState,
} from "./TimeDistTabs/TimeDistTabs.vue";

export interface State {
  speed: Speed;
  departure?: AirportFilterState;
  arrival?: AirportFilterState;
  timeDist?: TimeDistState;
}

interface ErrorStates extends ErrorState {
  speed: boolean;
  departure: boolean;
  arrival: boolean;
  timeDist: boolean;
}

@Component({ components: { SpeedInput, AirportFilters, TimeDistTabs } })
export default class FilterForm extends VueWithErrorCatcher<ErrorStates> {
  private state: State = {
    speed: new Speed("0.770", SpeedType.Mach),
  };

  constructor() {
    super({
      speed: false,
      departure: false,
      arrival: false,
      timeDist: false,
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
  margin-top: 50px;
  width: 60%;
  align-self: center;
  font-size: 3vh;
  padding: 5px;
}
</style>
