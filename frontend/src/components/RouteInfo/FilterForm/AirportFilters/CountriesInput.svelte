<script lang="ts">
  export let value: string = "";

  export function parse(): string[] | undefined {
    return value.length > 0 ? value.split(",") : undefined;
  }
</script>

<label for="countries">Countries</label>
<input name="countries" bind:value />
