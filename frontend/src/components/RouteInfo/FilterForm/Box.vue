<template>
  <div class="box">
    <label>{{ label }}</label>
    <div class="box-content">
      <slot />
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop } from "vue-property-decorator";

@Component
export default class Box extends Vue {
  @Prop({ required: true }) label!: string;
}
</script>

<style scoped>
.box {
  width: 15vw;
  align-self: center;
}

.box > label {
  text-align: center;
  display: block;
  padding-top: 25px;
  padding-bottom: 5px;
  font-size: 2.5vh;
}

.box-content {
  display: grid;
  row-gap: 10px;
  overflow: hidden;
  border: 1px solid var(--border-color);
  padding: 1.5vh;
}

.box-content >>> label {
  font-size: 2.25vh;
  padding-right: 15px;
}

.box-content >>> input,
select {
  font-size: 2.25vh;
}
</style>