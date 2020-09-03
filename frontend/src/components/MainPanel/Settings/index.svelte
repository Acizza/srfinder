<script lang="ts">
  import Theme from "./Theme.svelte";
</script>

<style>
  .settings-panel {
    display: flex;
    flex-direction: column;
    flex: 1 0;
  }

  h1 {
    border-bottom: 1px solid var(--border-color);
    text-align: center;
    justify-content: center;
    margin: 0;
    padding: 0.25em;
    font-weight: normal;
  }

  .settings-list {
    display: grid;
    row-gap: 0.25em;
    font-size: 1.4em;
    padding: 1em;
  }

  .settings-list :global(label) {
    grid-column: 1 / 2;
    align-self: center;
  }

  .settings-list :global(select) {
    grid-column: 2 / 3;
    font-size: 0.9em;
  }
</style>

<div class="settings-panel">
  <h1>Settings</h1>
  <div class="settings-list">
    <Theme />
  </div>
</div>
