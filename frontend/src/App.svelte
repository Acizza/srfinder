<script lang="ts">
  import RouteMap from "./components/RouteMap.svelte";
  import RouteInfo from "./components/RouteInfo/index.svelte";
</script>

<style>
  :global(body) {
    background-color: var(--bg-color);
    color: var(--text-color);
    font-family: Lekton;
    margin: 0;
  }

  :global(:root) {
    --hover-color: #3e3e41;
    --bg-color: #353538;
    --tab-color: #2e2e30;
    --border-color: #222222;

    --text-color: #d2d2d2;
  }

  :global(input, select) {
    background-color: #323436;
    border: 1px solid var(--border-color);
    color: var(--text-color);
    font-family: Lekton;
  }

  :global(input) {
    font-size: 1em;
  }

  :global(input[type="submit"]) {
    transition: background-color 200ms ease-in-out;
  }

  :global(input[type="submit"]:hover) {
    background-color: var(--hover-color);
  }

  main {
    width: 100%;
    height: 100%;
    display: flex;
  }
</style>

<main>
  <RouteMap />
  <RouteInfo />
</main>