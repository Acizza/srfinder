<template>
  <form-input label="ICAO" class="icao-input" maxlength="4" v-model="currentICAO" :error="error" />
</template>

<script lang="ts">
import { Component } from "vue-property-decorator";
import FormInput from "../FormInput.vue";
import "../../../../util/string";
import VueWithError from "../../../../util/vue_with_error";

const enum Error {
  NotAlphanumeric = "Must only contain digits and/or letters"
}

@Component({ components: { FormInput } })
export default class IcaoInput extends VueWithError<Error> {
  private icao = "";

  private get currentICAO(): string {
    return this.icao;
  }

  private set currentICAO(icao: string) {
    this.icao = icao.toUpperCase();

    const isValid = this.icao.allChars(
      ch => ch.isAlphanumericUpper() || ch.isDigit()
    );

    this.setError(isValid ? null : Error.NotAlphanumeric);

    this.$emit(
      "input",
      this.currentICAO.length > 0 ? this.currentICAO : undefined
    );
  }
}
</script>

<style scoped>
.icao-input >>> input {
  width: 5vh;
}
</style>
