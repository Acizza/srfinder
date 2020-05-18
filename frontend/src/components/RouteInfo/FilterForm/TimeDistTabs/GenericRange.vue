<template>
  <div class="range-inputs">
    <form-input
      label="Min"
      v-model="currentMinValue"
      :error="state.min.error"
    />
    <form-input
      label="Max"
      v-model="currentMaxValue"
      :error="state.max.error"
    />
  </div>
</template>

<script lang="ts">
import { Component, Prop } from "vue-property-decorator";
import FormInput from "../FormInput.vue";
import {
  VueWithErrorCatcher,
  ErrorState,
} from "../../../../util/vue_with_error";
import Range from "../../../../types/range";
import ErrorString from "../../../../types/error_string";

interface State {
  min: InputState;
  max: InputState;
}

interface InputState {
  value: string;
  error: ErrorString;
}

interface ErrorStates extends ErrorState {
  min: boolean;
  max: boolean;
}

@Component({ components: { FormInput } })
export default class GenericRange<T> extends VueWithErrorCatcher<ErrorStates> {
  @Prop({ required: true }) isInputValid!: (value: string) => ErrorString;
  @Prop({ required: true }) serialize!: (value: string) => T | undefined;

  private state: State = {
    min: {
      value: "",
      error: null,
    },
    max: {
      value: "",
      error: null,
    },
  };

  constructor() {
    super({ min: false, max: false });
  }

  private setInputState(variant: "min" | "max", value: string) {
    let state: InputState;

    switch (variant) {
      case "min":
        state = this.state.min;
        break;
      case "max":
        state = this.state.max;
        break;
    }

    state.value = value;
    state.error = this.isInputValid(value);

    this.setErrorAndPropagate(variant, state.error !== null);
    this.emitInput();
  }

  private emitInput() {
    const min = this.serialize(this.state.min.value);
    const max = this.serialize(this.state.max.value);

    let serialized: Range<T> | undefined = undefined;

    if (min !== undefined || max !== undefined) {
      serialized = { min, max };
    }

    this.$emit("input", serialized);
  }

  private get currentMinValue(): string {
    return this.state.min.value;
  }

  private set currentMinValue(value: string) {
    this.setInputState("min", value);
  }

  private get currentMaxValue(): string {
    return this.state.max.value;
  }

  private set currentMaxValue(value: string) {
    this.setInputState("max", value);
  }

  clear() {
    this.setInputState("min", "");
    this.setInputState("max", "");
  }
}
</script>

<style scoped>
.range-inputs {
  display: flex;
  flex-direction: column;
}

.range-inputs >>> .form-input {
  margin-left: 20px;
  margin-right: 20px;
  margin-bottom: 10px;
}

.range-inputs >>> .form-input .content-wrapper {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
}

.range-inputs >>> label {
  font-size: 1vw;
  padding-right: 10px;
}

.range-inputs >>> input {
  font-size: 1vw;
  min-width: 30%;
}
</style>
