<script lang="ts">
  import Input from "../../../Input.svelte";
  import type { InputResult } from "../../types";

  export function parse(): string | undefined {
    return value.length > 0 ? value : undefined;
  }

  let value = "";

  function validate(newValue: string): InputResult {
    if (newValue.length > 0 && !newValue.isAlphanumeric())
      return { kind: "err", value: "Can only contain A-Z and 0-9" };

    return { kind: "ok", value: newValue.toUpperCase() };
  }
</script>

<Input name="icao" label="ICAO" maxlength={4} {validate} bind:value />
