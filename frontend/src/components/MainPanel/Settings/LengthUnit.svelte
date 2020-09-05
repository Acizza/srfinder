<script lang="ts" context="module">
  const tooltip = "Used to display and filter runway lengths.";
</script>

<script lang="ts">
  import {
    curLengthUnit,
    LengthUnit,
    LengthUnitKind,
  } from "../../../settings/units";
  import Input from "../Input.svelte";

  const initialUnit = $curLengthUnit;

  let value: LengthUnitKind;

  function applySelected() {
    $curLengthUnit = value;
    LengthUnit.save();
  }
</script>

<Input name="length-unit" label="Length" {tooltip}>
  <select
    name="length-unit"
    bind:value
    on:change={applySelected}
    on:blur={applySelected}>
    {#each Object.entries(LengthUnitKind) as [display, value]}
      <option {value} selected={value === initialUnit}>{display}</option>
    {/each}
  </select>
</Input>
