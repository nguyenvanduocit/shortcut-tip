<template>
  <nav data-tauri-drag-region class="navbar navbar-expand navbar-light d-flex justify-content-between align-items-center" :class="$style.menubar">
    <slot>
      <AppMenuBarDefault/>
    </slot>
  </nav>

</template>

<script lang="ts" setup>

import AppMenuBarDefault from "./AppMenuBarDefault.vue";
</script>

<style lang="stylus" module>
.menubar
  min-height 50px
  --bs-navbar-padding-y 5px
  padding 0 10px 0 15px !important
</style>
