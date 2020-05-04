<template>
  <div v-show="isSelected" class="tab-content">
    <slot />
  </div>
</template>

<script lang="ts">
import { Component, Prop } from "vue-property-decorator";
import Vue from "vue";

@Component
export default class Tab extends Vue {
  @Prop({ required: true }) name!: string;
  @Prop({ default: false }) private selected!: boolean;

  isSelected: boolean = this.selected;
}
</script>
