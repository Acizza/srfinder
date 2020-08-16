<script lang="ts">
  import type { InputResult } from "./types";

  export let name: string;
  export let label: string;
  export let value: any;
  export let type: string = "text";

  export let validate: (input: string) => InputResult;

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

<label for={name}>{label}</label>
<input
  {name}
  {type}
  {value}
  bind:this={inputRef}
  on:input={handleInput}
  {...$$restProps} />
