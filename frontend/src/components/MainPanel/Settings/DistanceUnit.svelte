<script lang="ts" context="module">
  const tooltip =
    "Used to display the distance of a route and to specify the min / max distance filter.";

  function unitName(unit: DistanceUnitKind): string {
    switch (unit) {
      case DistanceUnitKind.NauticalMiles:
        return "Nautical Miles";
      case DistanceUnitKind.Miles:
        return "Miles";
      case DistanceUnitKind.Kilometers:
        return "Kilometers";
    }
  }

  const units = Object.keys(DistanceUnitKind).map((key) => {
    const unit: DistanceUnitKind = (DistanceUnitKind as any)[key];
    return { name: unitName(unit), value: unit };
  });
</script>

<script lang="ts">
  import {
    curDistanceUnit,
    DistanceUnit,
    DistanceUnitKind,
  } from "../../../settings/units";
  import Input from "../Input.svelte";

  const initialUnit = $curDistanceUnit;

  let value: DistanceUnitKind;

  function applySelected() {
    $curDistanceUnit = value;
    DistanceUnit.save();
  }
</script>

<Input name="distance-unit" label="Distance" {tooltip}>
  <select
    name="distance-unit"
    bind:value
    on:change={applySelected}
    on:blur={applySelected}>
    {#each units as { name, value }}
      <option {value} selected={value === initialUnit}>{name}</option>
    {/each}
  </select>
</Input>
