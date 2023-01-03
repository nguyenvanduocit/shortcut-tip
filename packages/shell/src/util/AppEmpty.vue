<template>
  <div class="card-body text-center">
    <div class="row justify-content-center">
      <div class="col-12 col-md-10 col-xl-8">

        <img src="../img/illustrations/lost.svg" alt="..." class="img-fluid mb-4" style="max-width: 272px;">
        <h2>
          No thing found
        </h2>
        <p class="text-muted">
          There are currently no data available.
        </p>
        <slot name="body"></slot>
      </div>
    </div> <!-- / .row -->
  </div>
</template>

<script lang="ts" setup>
</script>

<style lang="stylus" module>
</style>
