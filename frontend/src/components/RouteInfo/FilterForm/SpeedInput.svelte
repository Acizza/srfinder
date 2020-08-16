<script lang="ts">
  import type { ParsedSpeed, InputResult } from "./types";
  import { SpeedType } from "./types";
  import Input from "./Input.svelte";

  export let value: string = "0.77";
  export let parsed: ParsedSpeed | undefined = undefined;

  function validate(newValue: string): InputResult {
    const num = Number(newValue);

    if (Number.isNaN(num)) {
      parsed = undefined;
      return { kind: "err", value: "Must be a number." };
    }

    const type = num % 1 === 0 ? SpeedType.Knots : SpeedType.Mach;
    parsed = { value: num, type };

    return {
      kind: "ok",
      value: newValue,
    };
  }

  validate(value);
</script>

<style>
  :global(.speed-input) {
    font-size: 1.4em;
    margin-top: 0.5em;
    width: 3em;
  }
</style>

<svelte:options accessors />

<Input
  name="speed"
  label="Cruise Speed"
  class="speed-input"
  type="number"
  step="0.01"
  min="0.01"
  max="9999"
  bind:value
  {validate} />
