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
    --tab-color: #333333;
    --border-color: #222222;

    --text-color: #d2d2d2;
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
