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
              Payment methods
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
                  Recovery codes
                </h4>
                <small class="text-muted">
                  Standard messaging rates apply
                </small>
              </div>
              <div class="col-auto">
                <div class="form-check form-switch">
                  <input class="form-check-input" id="subscriptionsSwitchOne" type="checkbox" checked="">
                  <label class="form-check-label" for="subscriptionsSwitchOne"></label>
                </div>
              </div>
            </div> <!-- / .row -->
          </div>
          <div class="list-group-item">
            <div class="row align-items-center">
              <div class="col">

                <h4 class="mb-1">
                  Start with system
                </h4>
                <small class="text-muted">
                  Standard messaging rates apply
                </small>
              </div>
              <div class="col-auto">
                <input type="email" class="form-control" id="exampleInputEmail1" placeholder="Enter email">
              </div>
            </div> <!-- / .row -->
          </div>
        </div>
      </div>
      <div class="card-footer">
        <button type="submit" class="btn btn-primary">Save settings</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">

import {useStore} from "../module/projectList/composables/useStore";

const {clearProjects} = useStore()

</script>

<style module lang="stylus">
</style>
