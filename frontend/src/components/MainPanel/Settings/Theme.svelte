<script lang="ts">
  import { applyTheme, currentTheme, Theme } from "../../../theme";
  import Input from "../Input.svelte";

  const initialTheme = $currentTheme;

  let value: Theme;

  function applySelected() {
    applyTheme(value, true);
  }
</script>

<Input name="theme" label="Site Theme">
  <select name="theme" bind:value>
    {#each Object.entries(Theme) as [display, value]}
      <option
        {value}
        selected={value === initialTheme}
        on:click={applySelected}>
        {display}
      </option>
    {/each}
  </select>
</Input>
