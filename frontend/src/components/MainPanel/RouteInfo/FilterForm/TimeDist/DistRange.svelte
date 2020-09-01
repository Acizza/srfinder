<script lang="ts">
  import GenericRange from "./GenericRange.svelte";
  import type { InputResult, ParsedRange, TimeDistVariant } from "../../types";
  import { trimObject } from "../../util";

  let minValue: string = "";
  let maxValue: string = "";

  function validate(input: string): InputResult {
    const num = Number(input);

    if (Number.isNaN(num)) {
      return { kind: "err", value: "Input must be a number" };
    }

    return { kind: "ok", value: input };
  }

  export function parse(): [TimeDistVariant, ParsedRange<number>] | undefined {
    const minNum = Number(minValue);
    const maxNum = Number(maxValue);

    const result = trimObject({
      min: Number.isNaN(minNum) || minNum === 0 ? undefined : minNum,
      max: Number.isNaN(maxNum) || maxNum === 0 ? undefined : maxNum,
    });

    return result && ["dist", result];
  }

  function formatTooltip(variant: "Minimum" | "Maximum"): string {
    return `${variant} distance a route can be, in nautical miles.`;
  }
</script>

<GenericRange
  name="dist"
  minTooltip={formatTooltip('Minimum')}
  maxTooltip={formatTooltip('Maximum')}
  {validate}
  bind:minValue
  bind:maxValue />
