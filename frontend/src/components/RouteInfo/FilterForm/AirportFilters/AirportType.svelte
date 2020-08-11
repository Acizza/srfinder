<script lang="ts" context="module">
  const airportTypes: AirportTypes = {
    unknown: "",
    large_airport: "Large",
    medium_airport: "Medium",
    small_airport: "Small",
    closed: "Closed",
    heliport: "Heliport",
    seaplane_base: "Seaplane Base",
  };

  const typeNames = Object.keys(airportTypes).map((type) => {
    return {
      type,
      display: (airportTypes as any)[type],
    };
  });
</script>

<script lang="ts">
  import type { AirportTypes } from "../types";

  export let value: string;
</script>

<style>
  .type-selection {
    display: flex;
    flex-wrap: nowrap;
  }

  select {
    width: 5em;
    font-size: 0.75em;
  }
</style>

<div class="type-selection">
  <label for="type">Type</label>
  <select name="type" bind:value>
    {#each typeNames as { type, display }}
      <option value={type}>{display}</option>
    {/each}
  </select>
</div>
