<template>
  <div class="container-fluid p-3">
    <div class="card">
      <div class="card-header">
        <div class="row align-items-center">
          <div class="col">
            <!-- Title -->
            <h4 class="card-header-title">
              Projects
            </h4>
          </div>
        </div>
      </div>

      <div class="card-body">
        <button class="btn btn-danger" @click.prevent="clearProjects">Clear projects</button>
      </div>
    </div>

    <div class="card">
      <div class="card-header">
        <div class="row align-items-center">
          <div class="col">
            <!-- Title -->
            <h4 class="card-header-title">
              System
            </h4>
          </div>
        </div>
      </div>
      <div class="card-body">
        <div class="list-group list-group-flush my-n3">
          <div class="list-group-item">
            <div class="row align-items-center">
              <div class="col">

                <h4 class="mb-1">
                  Start with system
                </h4>
                <small class="text-muted">
                  Auto start the app with system
                </small>
              </div>
              <div class="col-auto">
                <div class="form-check form-switch">
                  <input class="form-check-input" id="subscriptionsSwitchOne" type="checkbox" :checked="isAutoStartEnabled" @change="toggleAutoStart">
                  <label class="form-check-label" for="subscriptionsSwitchOne"></label>
                </div>
              </div>
            </div> <!-- / .row -->
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">

import {useStore} from "../module/projectList/composables/useStore";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api";
import {tryOnMounted} from "@vueuse/core";

const {clearProjects} = useStore()

const isAutoStartEnabled = ref(false)

const toggleAutoStart = async (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.checked) {
    await enableAutoStart()
  } else {
    await disableAutoStart()
  }
}

const disableAutoStart = async () => {
  await invoke('plugin:autostart|disable')
  isAutoStartEnabled.value = false
}

const enableAutoStart = async () => {
  await invoke('plugin:autostart|enable')
  isAutoStartEnabled.value = true
}

const loadPreference = async () => {
  isAutoStartEnabled.value = await invoke('plugin:autostart|is_enabled')
}

tryOnMounted(() => {
  loadPreference()
})

</script>

<style module lang="stylus">
</style>
