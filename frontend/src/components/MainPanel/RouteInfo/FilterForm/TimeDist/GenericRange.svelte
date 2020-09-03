<script lang="ts">
  import Input from "../../../Input.svelte";
  import type { InputResult } from "../../types";

  export let name: string;

  export let minValue: string;
  export let maxValue: string;

  export let minTooltip: string | undefined = undefined;
  export let maxTooltip: string | undefined = undefined;

  export let validate: (input: string) => InputResult;
</script>

<Input
  name={`min-${name}`}
  label="Min"
  tooltip={minTooltip}
  {validate}
  bind:value={minValue} />
<Input
  name={`max-${name}`}
  label="Max"
  tooltip={maxTooltip}
  {validate}
  bind:value={maxValue} />
