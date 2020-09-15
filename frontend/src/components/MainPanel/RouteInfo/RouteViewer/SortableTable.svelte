<script lang="ts">
  import Icon from "fa-svelte";
  import { faAngleDown, faAngleUp } from "@fortawesome/free-solid-svg-icons";

  type Comparer = (a: any, b: any) => number;

  interface Header {
    name: string;
    index: number;
  }

  export let headers: Header[];
  export let data: any[];
  export let comparer: (headerIndex: number) => Comparer;

  export let className: string | undefined = undefined;

  let sortState: SortState | undefined = undefined;
  let sortedData: any[] = data;

  $: {
    if (sortState) {
      const { index, order } = sortState;
      const cmp = orderedComparer(order, comparer(index));
      const cloned = [...data];

      sortedData = cloned.sort(cmp);
    } else {
      sortedData = data;
    }
  }

  interface SortState {
    index: number;
    order: SortOrder;
  }

  const enum SortOrder {
    Ascending,
    Descending,
  }

  function sortOrderIcon(order: SortOrder): any {
    switch (order) {
      case SortOrder.Ascending:
        return faAngleUp;
      case SortOrder.Descending:
        return faAngleDown;
    }
  }

  function inverseSortOrder(order: SortOrder): SortOrder | undefined {
    switch (order) {
      case SortOrder.Ascending:
        return SortOrder.Descending;
      case SortOrder.Descending:
        return undefined;
    }
  }

  function orderedComparer(order: SortOrder, func: Comparer): Comparer {
    switch (order) {
      case SortOrder.Ascending:
        return func;
      case SortOrder.Descending:
        return (a, b) => func(b, a);
    }
  }

  function applySortOrder(header: Header) {
    const sameHeader = sortState && sortState.index === header.index;

    const order = sameHeader
      ? inverseSortOrder(sortState!.order)
      : SortOrder.Ascending;

    sortState =
      order !== undefined ? { index: header.index, order } : undefined;
  }
</script>

<style>
  table {
    border-spacing: 0;
  }

  table :global(.sort-icon) {
    width: 12px;
    height: 12px;
    vertical-align: middle;
  }

  thead {
    position: sticky;
    top: 0;
    background-color: var(--tab-color);
    cursor: pointer;
  }

  th {
    padding: 0.5em;
    font-weight: normal;
    border-bottom: 1px solid var(--border-color);
  }

  th:not(:last-child) {
    border-right: 1px solid var(--border-color);
  }
</style>

<table class={className}>
  <thead>
    {#each headers as header}
      <th on:click={() => applySortOrder(header)}>
        {header.name}
        {#if sortState && sortState.index === header.index}
          <Icon class="sort-icon" icon={sortOrderIcon(sortState.order)} />
        {/if}
      </th>
    {/each}
  </thead>
  <tbody>
    <slot {sortedData} />
  </tbody>
</table>
