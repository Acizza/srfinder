<script lang="ts">
  import Input from "../Input.svelte";
  import "../../../../util";

  const enum Selector {
    LessThan = "lt",
    Equal = "eq",
    GreaterThan = "gt",
  }

  export let value: string;

  function parseSelector(newValue: string): [Selector, string] {
    if (newValue.length === 0) {
      return [Selector.GreaterThan, newValue];
    }

    switch (newValue[0]) {
      case ">":
        return [Selector.GreaterThan, newValue.substr(1)];
      case "<":
        return [Selector.LessThan, newValue.substr(1)];
      default:
        return [Selector.Equal, newValue];
    }
  }
</script>

<style>
  :global(.runway-length-input) {
    width: 3em;
  }
</style>

<Input
  inputClass="runway-length-input"
  name="length"
  label="Length"
  bind:value />
