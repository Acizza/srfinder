<script lang="ts" context="module">
  const tooltip =
    "Required runway length.\n\n" +
    "Can be prefixed with < to exclude lengths less than the following number, or > for lengths greater than the following number.\n\n" +
    "Example: The value >12500 will filter out airports that don't have any runways that are at least 12,500 feet (default) / meters long.\n\n" +
    "The unit used to specify length can be changed in the settings panel on the bottom right.";
</script>

<script lang="ts">
  import "../../../../../util";
  import { LengthSelector } from "./types";
  import type { ParsedRunwayLength } from "./types";
  import type { InputResult } from "../../types";
  import Input from "../../../Input.svelte";
  import { LengthUnit } from "../../../../../settings/units";

  let parsed: ParsedRunwayLength | undefined = undefined;

  export function parse(): ParsedRunwayLength | undefined {
    if (!parsed) return undefined;

    return {
      length: LengthUnit.fromCurrent(parsed.length),
      selector: parsed.selector,
    };
  }

  function validate(newValue: string): InputResult {
    if (newValue.length === 0) {
      parsed = undefined;
      return { kind: "ok", value: newValue };
    }

    let selector: LengthSelector;
    let slice: string;

    switch (newValue[0]) {
      case ">":
        [selector, slice] = [LengthSelector.GreaterThan, newValue.substr(1)];
        break;
      case "<":
        [selector, slice] = [LengthSelector.LessThan, newValue.substr(1)];
        break;
      default:
        if (!newValue[0].isCharDigit()) {
          parsed = undefined;
          return { kind: "err", value: "Valid selectors are < and >" };
        }

        [selector, slice] = [LengthSelector.Equal, newValue];
        break;
    }

    const isSliceEmpty = slice.length === 0;

    if (!isSliceEmpty && !slice.isDigits()) {
      parsed = undefined;

      return {
        kind: "err",
        value: "Can only contain selector (< or >) and digits",
      };
    }

    parsed = {
      length: isSliceEmpty ? 0 : Number(slice),
      selector,
    };

    return { kind: "ok", value: newValue };
  }
</script>

<Input name="length" label="Length" {tooltip} {validate} value="" />
