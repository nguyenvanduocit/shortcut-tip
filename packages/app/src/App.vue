<script setup lang="ts">
import {NavSidebar, AppMenuBar, useUnListen} from '@aiocean/shell'
import {setMenus} from "@aiocean/shell/src/sidebar/store";
import {onMounted} from "vue";
import {emit} from "@tauri-apps/api/event";
import {useListen} from "@aiocean/shell/src/composable/useListen";
import {useRoute, useRouter} from "vue-router";
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

const router = useRouter()

useListen<string>("router::push", (e) => {
  router.push(e.payload);
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
