<script lang="ts">
  import GenericRange from "./GenericRange.svelte";
  import type { InputResult, ParsedRange, TimeDistVariant } from "../../types";
  import { trimObject } from "../../util";
  import { DistanceUnit } from "../../../../../settings/units";

  let minValue: string = "";
  let maxValue: string = "";

  function validate(input: string): InputResult {
    const num = Number(input);

    if (Number.isNaN(num)) {
      return { kind: "err", value: "Input must be a number" };
    }

    return { kind: "ok", value: input };
  }

  function validRangeValue(value: string): number | undefined {
    const num = Number(value);

    return Number.isNaN(num) || num === 0
      ? undefined
      : DistanceUnit.fromCurrent(num);
  }

  export function parse(): [TimeDistVariant, ParsedRange<number>] | undefined {
    const result = trimObject({
      min: validRangeValue(minValue),
      max: validRangeValue(maxValue),
    });

    return result && ["dist", result];
  }

  function formatTooltip(variant: "Minimum" | "Maximum"): string {
    return (
      `${variant} distance a route can be, in nautical miles (default) / miles / kilometers.\n\n` +
      "You can change which unit to use in the settings panel located in the bottom right."
    );
  }
</script>

<GenericRange
  name="dist"
  minTooltip={formatTooltip('Minimum')}
  maxTooltip={formatTooltip('Maximum')}
  {validate}
  bind:minValue
  bind:maxValue />
