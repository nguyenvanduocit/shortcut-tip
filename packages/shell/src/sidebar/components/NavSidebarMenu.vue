<template>
  <ul class="list-unstyled d-flex flex-column justify-content-center align-items-center w-100 mb-0" :class="$style.nav">
    <NavSidebarMenuItem v-for="menu in menus" :key="menu.id" :menu="menu"/>
  </ul>
</template>

<script lang="ts" setup>

import NavSidebarMenuItem from "./NavSidebarMenuItem.vue";
import {PropType} from "vue";
import {SidebarMenuItem} from "../type";

defineProps({
  menus: {
    type: Array as PropType<SidebarMenuItem[]>,
    required: true,
  },
})

</script>

<style lang="stylus" module>
.nav
  color #6e84a3
  :global(.nav-item)
    width 100%
  :global(.nav-link)
    display block
    text-align center
    padding-top 10px
    padding-bottom 10px
    position relative
    &:global(.active)
      color #12263f
      &::after
        content ""
        display block
        width 2px
        background #2c7be5
        position absolute
        left 0
        top .5rem
        bottom .5rem
</style>
