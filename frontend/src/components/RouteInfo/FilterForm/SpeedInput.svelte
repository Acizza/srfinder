<script lang="ts">
  export let value: string | undefined;
</script>

<style>
  input {
    font-size: 1.4em;
    margin-top: 0.5em;
    width: 3em;
  }
</style>

<label for="speed">Cruise Speed</label>
<input name="speed" type="number" step="0.001" bind:value />
