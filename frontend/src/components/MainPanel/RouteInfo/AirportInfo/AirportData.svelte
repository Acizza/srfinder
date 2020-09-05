<script lang="ts">
  import type { Airport, Runway } from "../types";
  import DataColumn from "./DataColumn.svelte";
  import Box from "../Box.svelte";
  import { hasAnyValues } from "../util";
  import {
    LengthUnitKind,
    LengthUnit,
    curLengthUnit,
  } from "../../../../settings/units";

  export let name: string;
  export let airport: Airport;

  let unitSuffix: string;

  function runwayName(runway: Runway): string {
    const hasBoth = runway.heMarker && runway.leMarker;

    if (hasBoth) return `${runway.heMarker!.name}-${runway.leMarker!.name}`;
    if (runway.heMarker) return runway.heMarker.name;
    if (runway.leMarker) return runway.leMarker.name;

    return "Unknown";
  }

  function runwaySize(runway: Runway): string | undefined {
    const length = runway.lengthFT && LengthUnit.toCurrent(runway.lengthFT);
    const width = runway.widthFT && LengthUnit.toCurrent(runway.widthFT);

    if (length) return width ? `${length}x${width}` : length.toString();

    return undefined;
  }

  function unitName(unit: LengthUnitKind): string {
    switch (unit) {
      case LengthUnitKind.Feet:
        return "FT";
      case LengthUnitKind.Meters:
        return "M";
    }
  }

  function formatFreq(freq: string | undefined): string | undefined {
    // Pad length of 7 ensures frequencies look as follows: 118.000
    return freq?.padEnd(7, "0") || undefined;
  }

  $: hasAnyFreqs = hasAnyValues(airport.frequencies);
  $: hasAnyRunways = hasAnyValues(airport.runways);

  $: {
    unitSuffix = unitName($curLengthUnit);
    // Since our units changed, trigger a redraw of the airport
    airport = airport;
  }
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
        <DataColumn label="ATIS" value={formatFreq(airport.frequencies.atis)} />
        <DataColumn
          label="GND"
          value={formatFreq(airport.frequencies.ground)} />
        <DataColumn label="TWR" value={formatFreq(airport.frequencies.tower)} />
        <DataColumn
          label="DEP"
          value={formatFreq(airport.frequencies.departure)} />
        <DataColumn
          label="DEP/ARR"
          value={formatFreq(airport.frequencies.arrivalDeparture)} />
        <DataColumn
          label="ARR"
          value={formatFreq(airport.frequencies.arrival)} />
        <DataColumn
          label="UNIC"
          value={formatFreq(airport.frequencies.unicom)} />
      </Box>
    {/if}
    {#if hasAnyRunways}
      <Box name="Runways" className="runways">
        {#each airport.runways as runway}
          <DataColumn
            label={runwayName(runway)}
            value={runwaySize(runway)}
            suffix={` ${unitSuffix}`} />
        {/each}
      </Box>
    {/if}
  {:else}
    <div class="message-container">
      <span class="no-airport-data">NO AIRPORT DATA</span>
    </div>
  {/if}
</div>
