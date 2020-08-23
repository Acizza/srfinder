<script lang="ts">
  export let error: string | null = null;
</script>

<style>
  .error-msg {
    padding: 0.5em;
    margin: 1em 1em 1.5em;
    border-radius: 5px;
    background-color: var(--error-color);
    box-shadow: -1px 1px 10px 1px rgba(63, 6, 6, 0.5);
    font-weight: bold;
  }

  .error-msg span {
    color: var(--error-text-color);
    font-size: 1.15em;
  }
</style>

<div class="error-msg">
  <span>{error}</span>
</div>
