<template>
  <div :class="$style.grid">
    <ProjectListItem
        v-for="project in projects"
        :key="project"
        :project="project"
        @delete="removeProject(project.keystroke)"
    />
  </div>

  <AppEmpty v-if="isEmpty" />
</template>

<script setup lang="ts">
import {useStore} from "../composables/useStore";
import ProjectListItem from "./ProjectListItem.vue";
import { AppEmpty } from '@aiocean/shell'
import {computed} from "vue";

const {projects, removeProject} = useStore()

const isEmpty = computed(() => Object.keys(projects.value).length === 0)

</script>


<style lang="stylus" module>
.grid
  display grid
  grid-auto-flow: column
  grid-auto-columns min-content
  grid-gap 10px
</style>
