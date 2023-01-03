<script setup lang="ts">
import {NavSidebar, AppMenuBar, useUnListen} from '@aiocean/shell'
import {setMenus} from "@aiocean/shell/src/sidebar/store";
import {computed, onMounted, onUnmounted} from "vue";
import {emit, listen} from "@tauri-apps/api/event";
import {useRoute} from "vue-router";
setMenus([
  {
    id: 'definition',
    name: 'Projects',
    icon: 'fe fe-globe',
    routePath: '/projects',
  }
])

onMounted(async () => {
  await emit("main::loaded")
})

</script>

<template>
  <NavSidebar/>
  <div class="main-content d-flex flex-column vh-100 vw-100 flex-grow-1 overflow-hidden">
    <AppMenuBar>
      <RouterView name="menuBar"/>
    </AppMenuBar>
    <div class="overflow-auto h-100 w-100">
      <RouterView/>
    </div>
  </div>
</template>
