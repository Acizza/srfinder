<script lang="ts">
  export let name: string;
</script>

<style>
  .box {
    display: flex;
    flex-direction: column;
    font-size: 1.1em;
  }

  span {
    text-align: center;
    margin-bottom: 0.5em;
    font-size: 1.25em;
  }

  .box-content {
    display: grid;
    flex-direction: column;
    row-gap: 1em;
    overflow: hidden;
    border: 1px solid var(--border-color);
    padding: 1em 0.75em;
    font-size: 1.1em;
  }

  :global(.box-content label) {
    grid-column: 1 / 2;
    padding-right: 0.5em;
    align-self: center;
  }

  :global(.box-content input, .box-content select) {
    grid-column: 2 / 3;
    max-width: 5em;
  }
</style>

<div class="box">
  <span>{name}</span>
  <div class="box-content">
    <slot />
  </div>
</div>
