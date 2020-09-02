<script lang="ts">
  import { version } from "../../../../package.json";
  import Icon from "fa-svelte";
  import { faCog } from "@fortawesome/free-solid-svg-icons";
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
  }

  footer :global(.settings) {
    width: 1.25em;
    height: 1.25em;
  }
</style>

<footer>
  <span>Version {version}</span>
  <Icon class="settings" icon={faCog} />
</footer>
