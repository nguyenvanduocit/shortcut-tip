<template>
  <div class="card">
    <div class="card-header px-3" :class="$style.header">
      <h4 class="card-header-title">
        {{project.keystroke}}
      </h4>
      <button class="btn-close" @click="$emit('delete')"></button>
    </div>
    <div class="card-body p-3">{{project.title}}</div>
  </div>
</template>
<script lang="ts" setup>

import {PropType} from "vue";

const props = defineProps({
  project: {
    type: Object as PropType<Shortcut>,
    required: true
  }
})

defineEmits(["delete"])

</script>

<style lang="stylus" module>
.header
  height 40px
</style>
