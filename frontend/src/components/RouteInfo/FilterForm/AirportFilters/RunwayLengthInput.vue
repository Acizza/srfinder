<template>
  <form-input
    label="Length"
    class="runway-length-input"
    v-model="currentLength"
    maxlength="6"
    :error="error"
  />
</template>

<script lang="ts">
import { Component } from "vue-property-decorator";
import VueWithError from "../../../../util/vue_with_error";
import FormInput from "../FormInput.vue";
import Result from "../../../../util/result";

const enum Error {
  InvalidSelector = "Must start with <, >, or number",
  MustBeNumber = "Must only contain numbers, excluding the selector",
}

const enum ParseError {
  Empty,
  InvalidDigits,
}

const enum LengthSelector {
  Equal = "eq",
  GreaterThan = "gt",
  LessThan = "lt",
}

export interface Serialized {
  length: number;
  selector: LengthSelector;
}

@Component({ components: { FormInput } })
export default class RunwayLengthInput extends VueWithError<Error> {
  private value = "";

  private get currentLength(): string {
    return this.value;
  }

  private set currentLength(value: string) {
    this.value = value;

    const parsed = RunwayLengthInput.parse(value);

    switch (parsed.kind) {
      case "ok":
        this.setError(null);
        this.$emit("input", parsed.value);
        break;
      case "err":
        switch (parsed.value) {
          case ParseError.Empty:
            this.setError(null);
            this.$emit("input", undefined);
            break;
          case ParseError.InvalidDigits:
            this.setError(Error.InvalidSelector);
            break;
        }
        break;
    }
  }

  private static parse(value: string): Result<Serialized, ParseError> {
    if (value.length === 0) return Result.err(ParseError.Empty);

    let type: LengthSelector;
    let slice: string;

    switch (value[0]) {
      case ">":
        type = LengthSelector.GreaterThan;
        slice = value.substr(1);
        break;
      case "<":
        type = LengthSelector.LessThan;
        slice = value.substr(1);
        break;
      default:
        type = LengthSelector.Equal;
        slice = value;
        break;
    }

    if (!slice.isDigits()) return Result.err(ParseError.InvalidDigits);

    return Result.ok({
      length: Number(slice),
      selector: type,
    });
  }
}
</script>

<style scoped>
.runway-length-input >>> input {
  width: 8vh;
}
</style>
