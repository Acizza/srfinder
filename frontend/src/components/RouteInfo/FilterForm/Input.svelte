<script lang="ts">
  export let name: string;
  export let label: string;
  export let value: string;

  export let maxLength: number | undefined = undefined;
  export let inputClass = "";
</script>

<style>
  .input-group {
    display: flex;
    flex-wrap: nowrap;
  }
</style>

<div class="input-group">
  <label for={name}>{label}</label>
  <input
    class={inputClass}
    {name}
    type="text"
    maxlength={maxLength}
    bind:value
    on:input />
</div>
