<template>
  <div class="container-fluid p-3">
    <ProjectList/>
  </div>
  <AppModal v-if="isShow" @close="close">
    <template #header>
      <h4 class="card-header-title">
        header
      </h4>
    </template>
    <form data-bitwarden-watching="1">
      <div class="form-group">
        <label for="exampleInputEmail1" class="form-label">Shortcut</label>
        <input type="text" class="form-control" v-model="keystroke">
      </div>
      <div class="form-group">
        <label for="exampleInputPassword1" class="form-label">Title</label>
        <input type="text" class="form-control" v-model="title">
      </div>

    </form>
    <template #footer>
      <div class="btn-group">
        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" @click.prevent="close">Close</button>
        <button type="button" class="btn btn-primary" @click.prevent="saveShortcut">Save changes</button>
      </div>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
import ProjectList from "../components/ProjectList.vue";
import {AppModal, useUnListen} from "@aiocean/shell"
import {useModal} from "@aiocean/shell"
import {listen} from "@tauri-apps/api/event";
import {ref} from "vue";
import {useStore} from "../composables/useStore";
const {addProject} = useStore()

const {isShow, close, show} = useModal()

useUnListen(async () => {
  return await listen("add-shortcut", (data) => show())
})

const keystroke = ref("")
const title = ref("")

const saveShortcut = () => {
  addProject(keystroke.value, title.value)
  keystroke.value = ""
  title.value = ""
  close()
}

</script>

<style module lang="stylus">
</style>
