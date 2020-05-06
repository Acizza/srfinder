<template>
  <form-input
    label="Cruise Speed"
    class="speed-input"
    :name="name"
    v-model="currentSpeed"
    :error="error"
  />
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import FormInput from "./FormInput.vue";

const enum SpeedType {
  Mach = "mach",
  Knots = "knots"
}

export class Speed {
  private constructor(public value: number, public type: SpeedType) {}

  static parse(value: number): Speed {
    const type = value % 1 === 0 ? SpeedType.Knots : SpeedType.Mach;
    return new Speed(value, type);
  }

  toJSON(): { value: number; type: SpeedType } {
    return {
      value: this.value,
      type: this.type
    };
  }
}

const enum Error {
  Empty = "Must not be empty",
  EqualsZero = "Must be above zero",
  NotNumber = "Must be a number"
}

@Component({ components: { FormInput } })
export default class SpeedInput extends Vue {
  @Prop({ required: true }) name!: string;

  private speed = "0.770";
  private error: Error | null = null;

  get currentSpeed(): string {
    return this.speed;
  }

  set currentSpeed(value: string) {
    this.speed = value;

    if (value.length === 0) {
      this.setError(Error.Empty);
      return;
    }

    const numValue = Number(value);

    if (Number.isNaN(numValue)) {
      this.setError(Error.NotNumber);
      return;
    }

    if (numValue < 0.01) {
      this.setError(Error.EqualsZero);
      return;
    }

    if (this.error !== null) this.setError(null);
  }

  private setError(error: Error | null) {
    this.error = error;
    this.$emit("error", this.error !== null);
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