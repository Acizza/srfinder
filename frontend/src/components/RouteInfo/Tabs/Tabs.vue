<template>
  <div class="tabs">
    <ul class="tab-list">
      <li
        v-for="tab in tabs"
        :key="tab.name"
        :class="{ 'selected-tab': tab.isSelected }"
        class="tab-header uppercase"
        @click="selectTab(tab)"
      >
        {{ tab.name }}
      </li>
    </ul>
    <slot />
  </div>
</template>

<script lang="ts">
import { Component } from "vue-property-decorator";
import Vue from "vue";
import Tab from "./Tab.vue";

@Component
export default class Tabs extends Vue {
  private tabs: Tab[] = [];
  private lastTab: Tab | null = null;

  mounted() {
    this.tabs = this.$children as Tab[];
    this.lastTab = this.tabs.find((tab) => tab.isSelected) || null;
  }

  private selectTab(tab: Tab) {
    if (this.lastTab) this.lastTab.isSelected = false;

    tab.isSelected = true;
    this.lastTab = tab;

    this.$emit("tab-changed", tab);
  }
}
</script>

<style scoped>
.tab-list {
  display: flex;
  justify-content: space-evenly;
  flex-wrap: nowrap;
  overflow: hidden;
  padding: 0;
  margin: 0;
  list-style: none;
  background-color: var(--tab-color);
  text-align: center;
}

.tab-header {
  cursor: default;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  font-size: 1vw;
  padding: 0.5vw;
  border-bottom: 1px solid var(--border-color);
}

.tab-header:not(.selected-tab) {
  transition: background-color 200ms ease-in-out;
}

.tab-header:hover {
  background-color: var(--hover-color);
}

.tab-header:not(:last-child) {
  border-right: 1px solid var(--border-color);
}

.tab-header.selected-tab {
  background-color: var(--bg-color);
  border-bottom: 0;
}

.tab-content {
  padding-top: 10px;
}
</style>
