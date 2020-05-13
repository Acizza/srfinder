<template>
  <form-input label="Cruise Speed" class="speed-input" v-model="currentSpeed" :error="error" />
</template>

<script lang="ts">
import { Component, Prop } from "vue-property-decorator";
import FormInput from "./FormInput.vue";
import VueWithError from "../../../util/vue_with_error";
import Result from "../../../util/result";

export const enum SpeedType {
  Mach = "mach",
  Knots = "knots",
}

const enum Error {
  Empty = "Must not be empty",
  EqualsZero = "Must be above zero",
  NotNumber = "Must be a number",
}

export class Speed {
  constructor(public value: string, public type: SpeedType) {}

  static parse(value: string): Result<Speed, Error> {
    if (value.length === 0) return Result.err(Error.Empty);

    const numValue = Number(value);

    if (Number.isNaN(numValue)) return Result.err(Error.NotNumber);
    if (numValue < 0.01) return Result.err(Error.EqualsZero);

    const type = numValue % 1 === 0 ? SpeedType.Knots : SpeedType.Mach;

    return Result.ok(new Speed(value, type));
  }

  toJSON(): { value: number; type: SpeedType } {
    return {
      value: Number(this.value),
      type: this.type,
    };
  }
}

@Component({ components: { FormInput } })
export default class SpeedInput extends VueWithError<Error> {
  @Prop({ required: true }) private initialSpeed!: Speed;

  private value = this.initialSpeed.value;

  get currentSpeed(): string {
    return this.value;
  }

  set currentSpeed(value: string) {
    this.value = value;

    const parsed = Speed.parse(value);

    switch (parsed.kind) {
      case "ok":
        this.setError(null);
        this.$emit("input", parsed.value);
        break;
      case "err":
        this.setError(parsed.value);
        break;
    }
  }
}
</script>

<style scoped>
.speed-input >>> .content-wrapper {
  display: flex;
  flex-direction: column;
  flex-wrap: nowrap;
}

.speed-input >>> label {
  text-align: center;
  padding-bottom: 5px;
  font-size: 2.75vh;
}

.speed-input >>> input {
  width: 7vh;
  align-self: center;
  padding-left: 5px;
  padding-right: 5px;
}
</style>
