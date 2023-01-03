<template>
  <div class="modal fade show" role="dialog" aria-modal="true" style="display: block;">
    <div class="modal-dialog modal-dialog-centered" role="document">
      <div class="modal-content">
        <div class="modal-card card">
          <div class="card-header">
            <slot name="header">
              <h4 class="card-header-title">
                header
              </h4>
            </slot>

            <!-- Close -->
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" @click.prevent="$emit('close')"></button>

          </div>
          <div class="card-body">
            <slot/>
          </div>
          <div class="card-footer">
            <slot name="footer"></slot>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="modal-backdrop fade show"></div>
</template>

<script lang="ts" setup>
defineEmits(["close"])
</script>

<style lang="stylus" module>
</style>
