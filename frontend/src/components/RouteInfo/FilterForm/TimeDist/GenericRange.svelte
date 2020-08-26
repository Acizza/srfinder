<script lang="ts">
  import Input from "../Input.svelte";
  import type { InputResult } from "../../types";

  export let name: string;

  export let minValue: string;
  export let maxValue: string;

  export let validate: (input: string) => InputResult;
</script>

<Input name={`min-${name}`} label="Min" {validate} bind:value={minValue} />
<Input name={`max-${name}`} label="Max" {validate} bind:value={maxValue} />
