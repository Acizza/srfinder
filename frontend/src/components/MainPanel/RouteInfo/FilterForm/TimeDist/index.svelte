<script lang="ts">
  import Tabs from "../../../Tabs/index.svelte";
  import Tab from "../../../Tabs/Tab.svelte";
  import DistRange from "./DistRange.svelte";
  import type { ParsedTimeDist } from "../../types";
  import TimeRange from "./TimeRange.svelte";

  let inputRef: any;

  export function parse(): ParsedTimeDist | undefined {
    const result = inputRef?.parse() || undefined;
    if (!result) return undefined;

    const [type, value] = result;

    return {
      type,
      value,
    };
  }
</script>

<style>
  .time-dist-container {
    display: flex;
    flex-direction: column;
    flex: 1 0;
    border: 1px solid var(--border-color);
    margin: 2em 1em;
  }

  .time-dist-content {
    display: grid;
    row-gap: 0.5em;
    overflow: hidden;
    padding: 0.5em;
    font-size: 1.2em;
  }

  :global(.time-dist-content label) {
    grid-column: 1 / 2;
    padding-right: 0.5em;
    align-self: center;
  }

  :global(.time-dist-content input) {
    grid-column: 2 / 3;
    max-width: 4em;
  }
</style>

<div class="time-dist-container">
  <Tabs headers={['TIME', 'DISTANCE']} canScroll={false}>
    <Tab alwaysRender={false}>
      <div class="time-dist-content">
        <TimeRange bind:this={inputRef} />
      </div>
    </Tab>

    <Tab alwaysRender={false}>
      <div class="time-dist-content">
        <DistRange bind:this={inputRef} />
      </div>
    </Tab>
  </Tabs>
</div>
