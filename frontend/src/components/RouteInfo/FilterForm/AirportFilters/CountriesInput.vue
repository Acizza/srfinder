<template>
  <form-input label="Countries" class="countries-input" @input="input" />
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";
import FormInput from "../FormInput.vue";

@Component({ components: { FormInput } })
export default class CountriesInput extends Vue {
  private input(value: string) {
    const countries = value.split(",").reduce((acc, value) => {
      const trimmed = value.trim();
      if (trimmed.length > 0) acc.push(trimmed);
      return acc;
    }, [] as string[]);

    const inputValue = countries.length > 0 ? countries : undefined;
    this.$emit("input", inputValue);
  }
}
</script>

<style scoped>
.countries-input >>> input {
  width: 7vmax;
}
</style>
