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
import Time from "../../../../types/time";

const enum Error {
  InvalidFormat = "Must be formatted as HH:MM",
  InvalidHour = "Hour must be less than 100",
  InvalidMinute = "Minutes must be less than 60",
}

function isValidFragment(
  value: string,
  minFirst: string,
  minSecond: string,
  oobError: Error
): ErrorString {
  if (!value.isDigits()) return Error.InvalidFormat;

  switch (value.length) {
    case 0:
      return null;
    case 1:
      return value[0] <= minFirst ? null : oobError;
    case 2:
      return value[0] <= minFirst && value[1] <= minSecond ? null : oobError;
    default:
      return Error.InvalidFormat;
  }
}

function isValidHourString(value: string): ErrorString {
  return isValidFragment(value, "9", "9", Error.InvalidHour);
}

function isValidMinuteString(value: string): ErrorString {
  return isValidFragment(value, "5", "9", Error.InvalidMinute);
}

@Component({ components: { GenericRange } })
export default class TimeRange extends VueWithErrorPropagator {
  @Ref() readonly rangeRef!: GenericRange<Time>;

  private isInputValid(input: string): ErrorString {
    if (input.length === 0) return null;

    const colonSplit = input.split(":");

    switch (colonSplit.length) {
      case 0:
        return null;
      case 1:
        return isValidHourString(input);
      case 2: {
        const error = isValidHourString(colonSplit[0]);
        if (error) return error;

        return isValidMinuteString(colonSplit[1]);
      }
      default:
        return Error.InvalidFormat;
    }
  }

  private serialize(value: string): Time | undefined {
    if (value.length === 0) return undefined;

    const colonSplit = value.split(":");

    const hour = Number(colonSplit[0]);
    if (Number.isNaN(hour)) return undefined;

    if (colonSplit.length < 2) {
      return {
        hour,
        minutes: 0,
      };
    }

    const minutes = Number(colonSplit[1]);
    if (Number.isNaN(minutes)) return undefined;

    return { hour, minutes };
  }

  private onHasError(hasError: boolean) {
    this.emitHasError(hasError);
  }

  clear() {
    this.rangeRef.clear();
  }
}
</script>
