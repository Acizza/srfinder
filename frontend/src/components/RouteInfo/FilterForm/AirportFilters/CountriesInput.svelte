<script lang="ts">
  import Input from "../Input.svelte";

  export let value: string;
</script>

<style>
  :global(.countries-input) {
    width: 4em;
  }
</style>

<Input
  inputClass="countries-input"
  name="countries"
  label="Countries"
  bind:value />
