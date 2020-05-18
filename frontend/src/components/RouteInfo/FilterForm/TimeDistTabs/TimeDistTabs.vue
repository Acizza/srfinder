<template>
  <tabs class="time-dist-tabs" @tab-changed="onTabChanged">
    <tab name="Time" :selected="true">
      <time-range
        ref="timeRangeRef"
        @input="rangeChanged('timeRange', $event)"
        @has-error="onHasError"
      />
    </tab>
    <tab name="Distance">
      <distance-range
        ref="distanceRangeRef"
        @input="rangeChanged('distance', $event)"
        @has-error="onHasError"
      />
    </tab>
  </tabs>
</template>

<script lang="ts">
import { VueWithErrorPropagator } from "../../../../util/vue_with_error";
import Range from "../../../../types/range";
import Time from "../../../../types/time";
import { Component, Ref } from "vue-property-decorator";
import Tabs from "../../Tabs/Tabs.vue";
import Tab from "../../Tabs/Tab.vue";
import TimeRange from "./TimeRange.vue";
import DistanceRange from "./DistanceRange.vue";

interface TimeRangeVariant {
  type: "timeRange";
  value: Range<Time>;
}

interface DistanceVariant {
  type: "distance";
  value: Range<number>;
}

export type Serialized = TimeRangeVariant | DistanceVariant | undefined;

@Component({ components: { Tabs, Tab, TimeRange, DistanceRange } })
export default class TimeDistTabs extends VueWithErrorPropagator {
  @Ref() readonly timeRangeRef!: TimeRange;
  @Ref() readonly distanceRangeRef!: DistanceRange;

  private onTabChanged() {
    this.timeRangeRef.clear();
    this.distanceRangeRef.clear();
  }

  private rangeChanged(
    variant: "timeRange" | "distance",
    value: Range<any> | undefined
  ) {
    let serialized = undefined;

    if (value !== undefined) {
      serialized = {
        type: variant,
        value,
      };
    }

    this.$emit("changed", serialized);
  }

  private onHasError(hasError: boolean) {
    this.emitHasError(hasError);
  }
}
</script>

<style scoped>
.time-dist-tabs {
  border: 1px solid var(--border-color);
  width: 10vw;
  align-self: center;
  margin-top: 50px;
}
</style>
