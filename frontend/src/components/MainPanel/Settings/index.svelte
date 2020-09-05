<script lang="ts">
  import Box from "../RouteInfo/Box.svelte";
  import DistanceUnit from "./DistanceUnit.svelte";
  import LengthUnit from "./LengthUnit.svelte";
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

  .misc-settings {
    display: grid;
    row-gap: 0.25em;
    font-size: 1.4em;
    padding: 1em;
  }

  .misc-settings :global(label) {
    grid-column: 1 / 2;
    align-self: center;
  }

  .misc-settings :global(select) {
    grid-column: 2 / 3;
    font-size: 0.9em;
  }

  .settings-panel :global(.unit-settings) {
    padding: 0.25em 2em;
  }
</style>

<div class="settings-panel">
  <h1>Settings</h1>
  <div class="misc-settings">
    <Theme />
  </div>
  <Box name="Units" className="unit-settings">
    <LengthUnit />
    <DistanceUnit />
  </Box>
</div>
