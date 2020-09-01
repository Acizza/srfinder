<script lang="ts">
  import GenericRange from "./GenericRange.svelte";
  import type {
    InputResult,
    ParsedRange,
    TimeDistVariant,
    Time,
  } from "../../types";
  import { trimObject } from "../../util";

  let minValue: string = "";
  let maxValue: string = "";

  function validate(input: string): InputResult {
    if (input.length === 0) return { kind: "ok", value: input };

    const time = parseTime(input);

    if (!time)
      return { kind: "err", value: "Input must be a time formatted as HH:MM" };

    if (time.hour > 999)
      return { kind: "err", value: "Hours must be less than 1000" };

    if (time.minutes > 59)
      return { kind: "err", value: "Minutes must be less than 60" };

    return { kind: "ok", value: input };
  }

  function parseTime(value: string): Time | undefined {
    const split = value.split(":");

    if (split.length !== 2 || !split[0].isDigits() || !split[1].isDigits())
      return undefined;

    const hour = Number(split[0]);
    const minutes = Number(split[1]);

    return {
      hour,
      minutes,
    };
  }

  export function parse(): [TimeDistVariant, ParsedRange<Time>] | undefined {
    const result = trimObject({
      min: parseTime(minValue),
      max: parseTime(maxValue),
    });

    return result && ["time", result];
  }

  function formatTooltip(variant: "Minimum" | "Maximum"): string {
    return `${variant} time a route can take to complete at the specified cruise speed, formatted as HH:MM.`;
  }
</script>

<GenericRange
  name="time"
  minTooltip={formatTooltip('Minimum')}
  maxTooltip={formatTooltip('Maximum')}
  {validate}
  bind:minValue
  bind:maxValue />
