<template>
  <generic-range
    ref="rangeRef"
    :serialize="serialize"
    :is-input-valid="isInputValid"
    @input="$emit('input', $event)"
    @has-error="onHasError"
  />
</template>

<script lang="ts">
import { Component, Ref } from "vue-property-decorator";
import GenericRange from "./GenericRange.vue";
import { VueWithErrorPropagator } from "../../../../util/vue_with_error";
import ErrorString from "../../../../types/error_string";

const enum Error {
  InvalidNumber = "Must be a number",
}

@Component({ components: { GenericRange } })
export default class DistanceRange extends VueWithErrorPropagator {
  @Ref() readonly rangeRef!: GenericRange<number>;

  private isInputValid(input: string): ErrorString {
    const numValue = Number(input);

    if (Number.isNaN(numValue)) return Error.InvalidNumber;

    return null;
  }

  private serialize(value: string): number | undefined {
    const result = Number(value);

    if (Number.isNaN(result)) return undefined;

    return result;
  }

  private onHasError(hasError: boolean) {
    this.emitHasError(hasError);
  }

  clear() {
    this.rangeRef.clear();
  }
}
</script>
