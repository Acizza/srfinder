<script lang="ts">
  import type { Airport, Runway, Frequencies } from "../types";
  import DataColumn from "./DataColumn.svelte";
  import Box from "../Box.svelte";
  import { hasAnyValues } from "../util";

  export let name: string;
  export let airport: Airport;

  function runwayName(runway: Runway): string {
    const hasBoth = runway.heMarker && runway.leMarker;

    if (hasBoth) return `${runway.heMarker!.name}-${runway.leMarker!.name}`;
    if (runway.heMarker) return runway.heMarker.name;
    if (runway.leMarker) return runway.leMarker.name;

    return "Unknown";
  }

  function runwaySize(runway: Runway): string | undefined {
    if (runway.lengthFT && runway.widthFT)
      return `${runway.lengthFT}x${runway.widthFT}`;

    if (runway.lengthFT) return runway.lengthFT.toString();

    return undefined;
  }

  function frequency<K extends keyof Frequencies>(freq: K): string | undefined {
    const value = airport.frequencies[freq];
    if (!value) return undefined;

    return value.padEnd(7, "0");
  }

  $: hasAnyFreqs = hasAnyValues(airport.frequencies);
  $: hasAnyRunways = hasAnyValues(airport.runways);
</script>

<style>
  .airport-data {
    display: flex;
    flex-direction: column;
    flex: 1 1;
  }

  .airport-name {
    text-align: center;
    font-size: 1.4em;
    border-bottom: 1px solid var(--border-color);
    padding: 1em 0 0.5em;
    margin-bottom: 0.5em;
  }

  :global(.airport-data .frequencies) {
    padding: 0.5em 3em;
  }

  :global(.airport-data .runways) {
    padding: 0.5em 2em;
  }

  .message-container {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    font-size: 1.4em;
  }
</style>

<div class="airport-data">
  <span class="airport-name">{name}</span>
  {#if hasAnyFreqs || hasAnyRunways}
    {#if hasAnyFreqs}
      <Box name="Frequencies" className="frequencies">
        <DataColumn label="ATIS" value={frequency('atis')} />
        <DataColumn label="GND" value={frequency('ground')} />
        <DataColumn label="TWR" value={frequency('tower')} />
        <DataColumn label="DEP" value={frequency('departure')} />
        <DataColumn label="DEP/ARR" value={frequency('arrivalDeparture')} />
        <DataColumn label="ARR" value={frequency('arrival')} />
        <DataColumn label="UNIC" value={frequency('unicom')} />
      </Box>
    {/if}
    {#if hasAnyRunways}
      <Box name="Runways" className="runways">
        {#each airport.runways as runway}
          <DataColumn
            label={runwayName(runway)}
            value={runwaySize(runway)}
            suffix={' FT'} />
        {/each}
      </Box>
    {/if}
  {:else}
    <div class="message-container">
      <span class="no-airport-data">NO AIRPORT DATA</span>
    </div>
  {/if}
</div>
