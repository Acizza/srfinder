<script lang="ts">
  import Box from "../../Box.svelte";
  import AirportType from "./AirportType.svelte";
  import IcaoInput from "./ICAOInput.svelte";
  import LengthInput from "./LengthInput.svelte";
  import CountriesInput from "./CountriesInput.svelte";
  import type { ParsedAirportFilters } from "../types";

  export let name: string;

  let icaoRef: any = null;
  let typeRef: any = null;
  let lengthRef: any = null;
  let countriesRef: any = null;

  export function parse(): ParsedAirportFilters | undefined {
    const result: ParsedAirportFilters = {
      icao: icaoRef?.parse(),
      type: typeRef?.parse(),
      length: lengthRef?.parsed,
      countries: countriesRef?.parse(),
    };

    const hasAnyValue = Object.values(result).some((val) => val !== undefined);

    return hasAnyValue ? result : undefined;
  }
</script>

<Box {name}>
  <IcaoInput bind:this={icaoRef} />
  <AirportType bind:this={typeRef} />
  <LengthInput bind:this={lengthRef} />
  <CountriesInput bind:this={countriesRef} />
</Box>
