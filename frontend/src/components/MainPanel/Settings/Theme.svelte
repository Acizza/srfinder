<script lang="ts">
  import { applySiteTheme, siteTheme, Theme } from "../../../settings/theme";
  import Input from "../Input.svelte";

  const initialTheme = $siteTheme;

  let value: Theme;

  function applySelected() {
    applySiteTheme(value, true);
  }
</script>

<Input name="theme" label="Site Theme">
  <select
    name="theme"
    bind:value
    on:change={applySelected}
    on:blur={applySelected}>
    {#each Object.entries(Theme) as [display, value]}
      <option {value} selected={value === initialTheme}>{display}</option>
    {/each}
  </select>
</Input>
