<script lang="ts">
  import SpeedInput from "./SpeedInput.svelte";
  import AirportFilters from "./AirportFilters/index.svelte";
  import type { AirportFiltersInput } from "./types";

  interface InputState {
    speed?: string;
    departure?: AirportFiltersInput;
    arrival?: AirportFiltersInput;
    timeDist?: any;
  }

  let state: InputState = {
    speed: "0.770",
  };

  $: {
    console.log(state);
  }
</script>

<style>
  .filter-form {
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    padding: 0 1em;
    align-items: center;
  }

  .find-routes-btn {
    margin-top: 2em;
    font-size: 1.5em;
    width: 60%;
    align-self: center;
  }

  :global(.filter-form .box:not(:first-child)) {
    margin-top: 1em;
  }

  :global(.filter-form > label) {
    font-size: 1.3em;
  }
</style>

<form class="filter-form">
  <SpeedInput bind:value={state.speed} />
  <AirportFilters name="Departure" bind:state={state.departure} />
  <AirportFilters name="Arrival" bind:state={state.arrival} />
  <input type="submit" class="find-routes-btn" value="Find Routes" />
</form>
