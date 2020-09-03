<script lang="ts">
  import { version } from "../../../package.json";
  import Icon from "fa-svelte";
  import { faCog } from "@fortawesome/free-solid-svg-icons";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
</script>

<style>
  footer {
    border-top: 1px solid var(--border-color);
    background-color: var(--tab-color);
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    padding: 0.5em;
    align-items: center;
    font-size: 0.75em;
    cursor: default;
  }

  .settings-wrapper {
    cursor: pointer;
  }

  footer :global(.settings-icon),
  .settings-wrapper {
    width: 1.25em;
    height: 1.25em;
  }
</style>

<footer>
  <span>Version {version}</span>
  <div class="settings-wrapper" on:click={() => dispatch('toggle-settings')}>
    <Icon class="settings-icon" icon={faCog} />
  </div>
</footer>
