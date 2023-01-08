<script setup lang="ts">
import {useListen} from "@aiocean/shell/src/composable/useListen";
import {ref} from "vue";

//const projects = store.projects
const projects = [
  {
    title: "Alfred App",
    keystroke: ["ctrl", "shift", "P"],
  },
  {
    title: "Build Terraform",
    keystroke: ["ctrl", "shift", "B"],
  },
  {
    title: "Goland",
    keystroke: ["ctrl", "shift", "A"],
  }
]

const shortcuts = ref<Shortcut[]|undefined>()

useListen<Shortcut[]>('shortcuts', (event) => {
  shortcuts.value = event.payload
})
</script>

<template>
  <div :class="$style.grid">
    <div v-for="project in projects" :class="$style.item">
      <div :class="$style.kbds">
        <kbd :class="$style.kbd">
          {{ project.keystroke[project.keystroke.length-1] }}
        </kbd>
      </div>
        <div :class="$style.title">
          {{project.title}}
        </div>
    </div>
  </div>
</template>
<style lang="stylus" module>
.grid
  display: flex;
.item
  background: #fff;
  border-radius: 5px;
  margin: 10px;
  padding 10px
  box-shadow: 1px 5px 5px rgba(0, 0, 0, 0.15);
  text-align center
.kbds
  white-space nowrap
.kbd
  font-family monospace
  display: inline-block;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 5px 10px;
  margin: 0 2px;
  box-shadow: 0 1px 0px rgba(0, 0, 0, 0.2), 0 0 0 2px #fff inset;
  background-color: #f7f7f7;
.title
  margin-top 5px
  font-size: 13px;
  font-weight: 400;
</style>
