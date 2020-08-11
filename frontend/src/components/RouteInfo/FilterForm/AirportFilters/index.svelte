<script lang="ts">
  import Box from "../../Box.svelte";
  import type { AirportFiltersInput } from "../types";
  import AirportType from "./AirportType.svelte";
  import IcaoInput from "./ICAOInput.svelte";
  import LengthInput from "./LengthInput.svelte";
  import CountriesInput from "./CountriesInput.svelte";

  export let name: string;

  export let state: AirportFiltersInput = {
    icao: "",
    type: "unknown",
    length: "",
    countries: "",
  };
</script>

<Box {name}>
  <IcaoInput bind:value={state.icao} />
  <AirportType bind:value={state.type} />
  <LengthInput bind:value={state.length} />
  <CountriesInput bind:value={state.countries} />
</Box>
