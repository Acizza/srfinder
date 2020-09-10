<script lang="ts">
  import SpeedInput from "./SpeedInput.svelte";
  import AirportFilters from "./AirportFilters/index.svelte";
  import type { FindRoutesQuery } from "../types";
  import { createEventDispatcher } from "svelte";
  import ErrorMessage from "./ErrorMessage.svelte";
  import TimeDist from "./TimeDist/index.svelte";

  export let error: string | null = null;
  export let loadingRoutes: boolean;

  let speedRef: any = null;
  let departureRef: any = null;
  let arrivalRef: any = null;
  let timeDistRef: any = null;

  const dispatch = createEventDispatcher();

  function filtersSubmitted() {
    const speed = speedRef?.parse();
    if (!speed) return;

    const query: FindRoutesQuery = {
      speed,
      departure: departureRef?.parse() || undefined,
      arrival: arrivalRef?.parse() || undefined,
      timeDist: timeDistRef?.parse() || undefined,
    };

    dispatch("findroutes", query);
  }
</script>

<style>
  .filter-form {
    margin-top: 1em;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    padding: 0 1em;
    align-items: center;
  }

  .find-routes-btn {
    margin-bottom: 1em;
    padding: 0.25em;
    font-size: 1.4em;
    align-self: center;
    cursor: pointer;
  }

  .find-routes-btn:disabled {
    color: var(--disabled-text-color);
    background-color: var(--disabled-color);
  }

  :global(.filter-form .box:not(:first-child)) {
    margin-top: 1em;
  }

  :global(.filter-form > label) {
    font-size: 1.3em;
  }
</style>

{#if error}
  <ErrorMessage {error} />
{/if}

<form class="filter-form" on:submit|preventDefault={filtersSubmitted}>
  <SpeedInput bind:this={speedRef} />
  <AirportFilters name="Departure" bind:this={departureRef} />
  <AirportFilters name="Arrival" bind:this={arrivalRef} />
  <TimeDist bind:this={timeDistRef} />
  <input
    type="submit"
    class="find-routes-btn"
    value={loadingRoutes ? 'Searching..' : 'Find Routes'}
    disabled={loadingRoutes} />
</form>
