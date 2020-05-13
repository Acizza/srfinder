<template>
  <box class="airport-filters" :label="label">
    <icao-input
      @input="set('icao', $event)"
      @has-error="setErrorAndPropagate('icao', $event)"
    />
    <airport-type-input @change="set('airportType', $event)" />
    <runway-length-input
      @input="set('runwayLength', $event)"
      @has-error="setErrorAndPropagate('runwayLength', $event)"
    />
    <countries-input @input="set('countries', $event)" />
  </box>
</template>

<script lang="ts">
import { Component, Prop } from "vue-property-decorator";
import {
  VueWithErrorCatcher,
  ErrorState,
} from "../../../../util/vue_with_error";
import Box from "../Box.vue";
import IcaoInput from "./IcaoInput.vue";
import AirportTypeInput from "./AirportTypeInput.vue";
import RunwayLengthInput, {
  Serialized as RunwayLength,
} from "./RunwayLengthInput.vue";
import CountriesInput from "./CountriesInput.vue";

export interface State {
  icao?: string;
  airportType?: string;
  runwayLength?: RunwayLength;
  countries?: string[];
}

interface ErrorStates extends ErrorState {
  icao: boolean;
  runwayLength: boolean;
}

@Component({
  components: {
    Box,
    IcaoInput,
    AirportTypeInput,
    RunwayLengthInput,
    CountriesInput,
  },
})
export default class AirportFilters extends VueWithErrorCatcher<ErrorStates> {
  @Prop({ required: true }) label!: string;

  private state: State = {};

  constructor() {
    super({
      icao: false,
      runwayLength: false,
    });
  }

  private set<T extends keyof State>(field: T, value: any) {
    this.state[field] = value;
    this.$emit("changed", this.state);
  }
}
</script>
