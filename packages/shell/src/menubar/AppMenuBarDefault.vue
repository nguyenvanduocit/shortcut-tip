<template>
  <h3 class="header-title" v-text="title"></h3>
</template>

<script lang="ts" setup>
import {useRoute} from "vue-router";
import {computed} from "vue";

const route = useRoute()

const title = computed(() => {
  return route.meta['title'] || capitalize(route.path.split('/').pop())
})

const capitalize = (s: string) => {
  return s.charAt(0).toUpperCase() + s.slice(1)
}
</script>

<style lang="stylus" module>
</style>
