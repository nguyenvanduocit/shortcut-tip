<script setup lang="ts">
import {useListen} from "@aiocean/shell/src/composable/useListen";
import {computed, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api";

const shortcuts = [
  {
    title: "Alfred",
    mods: ["ctrl", "shift", "alt"],
    keystroke: ["A"],
  },
  {
    title: "Build Terraform",
    mods: ["ctrl", "shift", "alt"],
    keystroke: ["B"],
  },
  {
    title: "Goland",
    mods: ["ctrl", "shift", "alt"],
    keystroke: ["G"],
  },
  {
    title: "Warp",
    mods: ["ctrl", "shift", "alt"],
    keystroke: ["W"],
  },
  {
    title: "Lock",
    mods: ["ctrl", "shift", "alt"],
    keystroke: ["L"],
  }
]

const modState = ref<ModState|undefined>()

useListen<ModState>('shortcuts', (event) => {
  modState.value = event.payload
})


const filteredShortcuts = computed(() => {
  if (!modState.value) {
    return shortcuts
  }

  return shortcuts.filter((shortcut) => {
    
    return shortcut.mods.every((mod) => {
      return (mod === "meta" && modState.value?.meta) || (mod === "ctrl" && modState.value?.ctrl) || (mod === "shift" && modState.value?.shift) || (mod === "alt" && modState.value?.alt)

    })
  })
})

const hasShortcuts = computed(() => {
  return filteredShortcuts.value.length > 0
})

watch(hasShortcuts, (value) => {
  if (value) {
    invoke('show_viewer')
  } else {
    invoke('hide_viewer')
  }
})

</script>

<template>
  <div :class="$style.grid">
    <div v-for="shortcut in filteredShortcuts" :class="$style.item">
      <div :class="$style.kbds">
        <kbd :class="$style.kbd">
          {{ shortcut.keystroke[shortcut.keystroke.length-1] }}
        </kbd>
      </div>
        <div :class="$style.title">
          {{shortcut.title}}
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
