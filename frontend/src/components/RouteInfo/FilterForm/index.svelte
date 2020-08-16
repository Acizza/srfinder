<script lang="ts">
  import SpeedInput from "./SpeedInput.svelte";
  import AirportFilters from "./AirportFilters/index.svelte";
  import type { FindRoutesQuery } from "../types";
  import { createEventDispatcher } from "svelte";
  import ErrorMessage from "./ErrorMessage.svelte";

  export let error: string | null = null;
  export let isLoadingRoutes: boolean;

  let speedRef: any = null;
  let departureRef: any = null;
  let arrivalRef: any = null;

  const dispatch = createEventDispatcher();

  function filtersSubmitted() {
    const speed = speedRef?.parsed;

    if (!speed) return;

    const query: FindRoutesQuery = {
      speed,
      departure: departureRef?.parse() || undefined,
      arrival: arrivalRef?.parse() || undefined,
      // timeDist
    };

    dispatch("findroutes", query);
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
    padding: 0.25em;
    font-size: 1.5em;
    width: 60%;
    align-self: center;
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
  <input
    type="submit"
    class="find-routes-btn"
    value={isLoadingRoutes ? 'Searching..' : 'Find Routes'}
    disabled={isLoadingRoutes} />
</form>
