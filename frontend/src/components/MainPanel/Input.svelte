<script lang="ts">
  import type { InputResult } from "./RouteInfo/types";

  export let name: string;
  export let label: string;
  export let value: any = "";

  export let tooltip: string | undefined = undefined;

  export let validate: (input: string) => InputResult = (input) => {
    return { kind: "ok", value: input };
  };

  let inputRef: HTMLInputElement | null;

  function handleInput(event: any) {
    const newValue = event.target.value;

    const result = validate(newValue);
    let error: string;

    switch (result.kind) {
      case "ok":
        value = result.value;
        error = "";
        break;
      case "err":
        value = newValue;
        error = result.value;
        break;
    }

    inputRef?.setCustomValidity(error);
  }
</script>

<style>
  label {
    display: flex;
  }

  .tooltip {
    font-family: Tahoma, Geneva, sans-serif;
    font-weight: bold;
    font-size: 0.5em;
    text-align: center;
    align-self: center;
    padding: 0.15em;
    margin-left: 0.5em;
    border-radius: 25%;
    color: var(--help-text-color);
    background-color: var(--help-text-background-color);
  }
</style>

<label for={name} title={tooltip}>{label}
  {#if tooltip}<span class="tooltip">?</span>{/if}</label>
<slot>
  <input
    {name}
    {value}
    type="text"
    bind:this={inputRef}
    on:input={handleInput}
    {...$$restProps} />
</slot>
