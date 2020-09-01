<script lang="ts">
  import type { InputResult } from "../../types";

  import Input from "../Input.svelte";

  export let value: string = "";

  export function parse(): string[] | undefined {
    return value.length > 0 ? value.split(",") : undefined;
  }

  function validate(input: string): InputResult {
    return { kind: "ok", value: input };
  }
</script>

<Input
  name="countries"
  label="Countries"
  tooltip="Comma-separated list of countries to search for airports in."
  bind:value
  {validate} />
