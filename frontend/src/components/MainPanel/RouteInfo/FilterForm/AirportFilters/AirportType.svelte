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
  import Input from "../../../Input.svelte";
  import type { AirportTypes, AirportType } from "./types";

  export let value: AirportType = "unknown";

  export function parse(): AirportType | undefined {
    return value !== "unknown" ? value : undefined;
  }
</script>

<Input name="type" label="Type" tooltip="Airport size / type to include.">
  <select name="type" bind:value>
    {#each typeNames as { type, display }}
      <option value={type}>{display}</option>
    {/each}
  </select>
</Input>
