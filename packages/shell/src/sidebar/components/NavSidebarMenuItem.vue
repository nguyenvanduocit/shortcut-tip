<template>
  <li class="nav-item" :class="$style.menuItem">
    <a v-if="menu.children" class="nav-link" :href="'#' + menu.id" data-bs-toggle="collapse" role="button" :aria-controls="menu.id">
      <i v-if="menu.icon" :class="menu.icon"></i>
    </a>
    <RouterLink v-else :to="menu.routePath ?? ''" active-class="active" class="nav-link">
      <i v-if="menu.icon" :class="menu.icon"></i>
    </RouterLink>

    <div class="collapse" :id="menu.id" v-if="menu.children">
      <ul class="nav nav-sm flex-column">
        <NavSidebarMenuItem v-for="childMenu in menu.children" :key="childMenu.id" :menu="childMenu"/>
      </ul>
    </div>
  </li>
</template>

<script lang="ts" setup>

import {PropType} from "vue";
import {SidebarMenuItem} from "../type";

defineProps({
  menu: {
    type: Object as PropType<SidebarMenuItem>,
    default: () => ({})
  }
});
</script>

<style lang="stylus" module>
.menuItem a
  user-select: none;
  -webkit-user-drag: none;
</style>
