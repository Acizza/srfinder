<template>
  <div class="form-input" :class="{ 'has-error': error !== null }">
    <div class="content-wrapper">
      <label>{{ label }}</label>
      <input
        type="text"
        :maxlength="maxlength"
        :value="value"
        @input="$emit('input', $event.target.value)"
      />
    </div>
    <div class="error-message uppercase" v-if="error">
      <span>{{ error }}</span>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import ErrorString from "../../../types/error_string";

@Component
export default class FormInput extends Vue {
  @Prop({ required: true }) label!: string;
  @Prop() maxlength!: number;
  @Prop() value!: string;
  @Prop({ default: null }) error!: ErrorString;
}
</script>

<style scoped>
.form-input.has-error {
  border: 2px solid #6e1111;
}

.form-input.has-error .content-wrapper {
  border: 3px solid #801919;
  padding: 0.5vmax;
}

.error-message {
  padding: 2px;
  background-color: #801919;
  text-align: center;
  font-size: 1vmax;
}
</style>
