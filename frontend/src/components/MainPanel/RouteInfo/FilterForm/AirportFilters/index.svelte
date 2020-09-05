<script lang="ts">
  import Box from "../../Box.svelte";
  import AirportType from "./AirportType.svelte";
  import IcaoInput from "./ICAOInput.svelte";
  import LengthInput from "./LengthInput.svelte";
  import CountriesInput from "./CountriesInput.svelte";
  import type { ParsedAirportFilters } from "./types";
  import { trimObject } from "../../util";

  export let name: string;

  let icaoRef: any = null;
  let typeRef: any = null;
  let lengthRef: any = null;
  let countriesRef: any = null;

  export function parse(): ParsedAirportFilters | undefined {
    return trimObject({
      icao: icaoRef?.parse(),
      airportType: typeRef?.parse(),
      runwayLength: lengthRef?.parse(),
      countries: countriesRef?.parse(),
    });
  }
</script>

<Box {name}>
  <IcaoInput bind:this={icaoRef} />
  <AirportType bind:this={typeRef} />
  <LengthInput bind:this={lengthRef} />
  <CountriesInput bind:this={countriesRef} />
</Box>
