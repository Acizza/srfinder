<script lang="ts">
  export let label: string;
  export let value: any;
  export let suffix: string = "";
</script>

<style>
  .column {
    display: grid;
    text-align: center;
  }

  .column .label {
    grid-column: 1 / 2;
    align-self: center;
    padding-right: 0.5em;
  }

  .column .value {
    grid-column: 2 / 3;
    font-style: italic;
  }
</style>

{#if value}
  <div class="column">
    <span class="label">{label}</span>
    <span class="value">{value}{suffix}</span>
  </div>
{/if}
