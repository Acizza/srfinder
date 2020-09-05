<script lang="ts">
  import { curTheme, ThemeKind, Theme } from "../../../settings/theme";
  import Input from "../Input.svelte";

  const initialTheme = $curTheme;

  let value: ThemeKind;

  function applySelected() {
    Theme.apply(value);
    Theme.save();
  }
</script>

<Input name="theme" label="Site Theme">
  <select
    name="theme"
    bind:value
    on:change={applySelected}
    on:blur={applySelected}>
    {#each Object.entries(ThemeKind) as [display, value]}
      <option {value} selected={value === initialTheme}>{display}</option>
    {/each}
  </select>
</Input>
