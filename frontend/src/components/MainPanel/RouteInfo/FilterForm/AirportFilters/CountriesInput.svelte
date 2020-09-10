<script lang="ts">
  import Input from "../../../Input.svelte";

  export let value: string = "";

  export function parse(): string[] | undefined {
    return value.length > 0
      ? value.split(",").map((str) => str.trim())
      : undefined;
  }
</script>

<Input
  name="countries"
  label="Countries"
  tooltip="Comma-separated list of countries to search for airports in."
  bind:value />
