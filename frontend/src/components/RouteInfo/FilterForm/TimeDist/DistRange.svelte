<script lang="ts">
  import GenericRange from "./GenericRange.svelte";
  import type { InputResult, ParsedRange, TimeDistVariant } from "../../types";

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

    const result = {
      min: Number.isNaN(minNum) || minNum === 0 ? undefined : minNum,
      max: Number.isNaN(maxNum) || maxNum === 0 ? undefined : maxNum,
    };

    const hasAnyValue = Object.values(result).some((val) => val !== undefined);

    return hasAnyValue ? ["dist", result] : undefined;
  }
</script>

<GenericRange name="dist" {validate} bind:minValue bind:maxValue />
