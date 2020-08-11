<script lang="ts">
  import Input from "../Input.svelte";

  export let value: string;

  function onInput(event: any) {
    value = event.target.value.toUpperCase();
  }
</script>

<style>
  :global(.icao-input) {
    width: 2em;
  }
</style>

<Input
  inputClass="icao-input"
  name="icao"
  label="ICAO"
  maxLength={4}
  on:input={onInput}
  {value} />
