<template>
  <b-modal body-class="pl-0 pr-0 pt-0 pb-0"
           centered
           hide-footer
           hide-header
           id="contactInfo-modal"
           style="direction: ltr;">
    <b-card class="overflow-hidden"
            no-body
            style="max-width: 540px;direction: ltr;border-color: #343a40 !important;">
      <div class="row">
        <div class="col pr-0 mr-0 my-logo"
             style="background-color: #343a40">

          <img height="275"
               src="../assets/logo_transparent.png"
               width="275"/>
        </div>
        <div class="col pl-0 pr-0  mr-3">
          <b-card
                  class="pr-custom"
                  style="font-family: Helvetica, Arial, sans-serif"
                  header="Contact Info"
                  header-bg-variant="light"
                  header-text-variant="black"
                  >
            <b-card-text class="mt-5 ml-5">
              <font-awesome-icon :icon="['fab', 'github']"
                                 class="mr-1"></font-awesome-icon>
              <a href="https://github.com/Vladimir-pa/">Github</a>
            </b-card-text>
            <b-card-text class="ml-5">
              <font-awesome-icon :icon="['fab', 'linkedin']"
                                 class="mr-1"></font-awesome-icon>
              <a href="https://www.linkedin.com/in/vladimir-parakhin/">LinkedIn</a>
            </b-card-text>
            <b-card-text class="ml-5 mb-3 pb-3">
              <font-awesome-icon class="mr-1"
                                 icon="envelope"></font-awesome-icon>
              <a href="vov4ikpa@gmail.com">Email </a>
            </b-card-text>
          </b-card>
        </div>
      </div>
    </b-card>
  </b-modal>
</template>

<script>
    export default {
        name: 'my-info-card'
    }
</script>

<style scoped>
  .card-header{
    font-size: larger;;
    font-weight: bolder;;
  }
  .my-logo{
    border-right-color: red !important;
  }
</style>