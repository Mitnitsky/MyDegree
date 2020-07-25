<template>
  <b-modal
    id="contactInfo-modal"
    body-class="pl-0 pr-0 pt-0 pb-0"
    centered
    hide-footer
    hide-header
    style="direction: ltr;"
  >
    <b-card
      class="overflow-hidden"
      no-body
      style="max-width: 540px;direction: ltr;border-color: #343a40 !important;"
    >
      <div class="row">
        <div
          class="col pr-0 mr-0 my-logo"
          style="background-color: #343a40"
        >
          <img
            height="275"
            src="../assets/logo_transparent.png"
            width="275"
          >
        </div>
        <div class="col pl-0 pr-0  mr-3">
          <b-card
            class="pr-custom "
            header="Contact Info"
            header-bg-variant="light"
            header-text-variant="black"
            style="font-family: Helvetica, Arial, sans-serif"
          >
            <b-card-body class="justify-content-center">
              <b-card-text class="ml-4 pt-4 pb-2">
                <font-awesome-icon
                  :icon="['fab', 'github']"
                  class="mr-1"
                />
                <a
                  href="https://github.com/Vladimir-pa/"
                  target="_blank"
                >Github</a>
              </b-card-text>
              <b-card-text class="ml-4 pb-5">
                <font-awesome-icon
                  :icon="['fab', 'linkedin']"
                  class="mr-1"
                />
                <a
                  href="https://www.linkedin.com/in/vladimir-parakhin/"
                  target="_blank"
                >LinkedIn</a>
              </b-card-text>
            </b-card-body>
          </b-card>
        </div>
      </div>
    </b-card>
  </b-modal>
</template>

<script>
export default {
  name: "MyInfoCard"
};
</script>

<style scoped>
.my-logo {
  border-right-color: red !important;
}
</style>
