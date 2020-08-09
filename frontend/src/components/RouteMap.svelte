<script lang="ts">
  import Spinner from "./Spinner.svelte";

  export let loaded: boolean = false;

  let map: any = undefined;
</script>

<style>
  .route-map {
    display: flex;
    flex: 1 1 0;
  }
</style>

<div class="route-map">
  {#if loaded}
    <div id="map-container" bind:this={map} />
  {:else}
    <Spinner />
  {/if}
</div>
