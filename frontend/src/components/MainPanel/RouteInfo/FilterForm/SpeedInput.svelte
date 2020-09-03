<script lang="ts">
  import type { ParsedSpeed } from "./AirportFilters/types";
  import { SpeedType } from "./AirportFilters/types";
  import type { InputResult } from "../types";
  import Input from "../../Input.svelte";

  export let value: string = "0.77";

  function validate(input: string): InputResult {
    if (input.length === 0)
      return { kind: "err", value: "Speed must be specified" };

    const num = Number(input);

    if (Number.isNaN(num)) return { kind: "err", value: "Must be a number" };

    if (num < 0.01) return { kind: "err", value: "Must be greater than 0.01" };
    if (num > 99999) return { kind: "err", value: "Must be less than 9999" };

    return {
      kind: "ok",
      value: input,
    };
  }

  export function parse(): ParsedSpeed | undefined {
    const num = Number(value);

    if (Number.isNaN(num)) return undefined;

    // We need to look for a '.' in the value to check for a floating point number as checking the remainder via
    // value % 1 === 0 will not catch numbers like 2.0 (which we want to interpret as a mach speed)
    const type = value.includes(".") ? SpeedType.Mach : SpeedType.Knots;

    return {
      type,
      value: num,
    };
  }
</script>

<style>
  :global(.speed-input) {
    font-size: 1.4em;
    margin-top: 0.5em;
    width: 3em;
  }
</style>

<Input
  name="speed"
  label="Cruise Speed"
  tooltip="Use whole numbers to indicate knots, and decimal numbers to indicate mach."
  class="speed-input"
  bind:value
  {validate} />
