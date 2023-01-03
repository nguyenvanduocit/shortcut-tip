<template>
  <h3 class="header-title">Help</h3>
  <button class="btn btn-primary btn-sm" @click.prevent="openFacebookPage"> <i class="fe fe-phone-call"></i> Contact us</button>
</template>

<script lang="ts" setup>
import { open } from '@tauri-apps/api/shell';
const openFacebookPage = () => {
  open("https://www.facebook.com/aiocean.io/")
}

</script>

<style lang="stylus" module>
</style>
