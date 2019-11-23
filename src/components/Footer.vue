<template>
  <div>
    <b-navbar fixed="bottom"
              type="dark"
              variant="dark">
      <div class="mx-auto"
           style="color: lightgray"
           @click="$bvModal.show('contactInfo-modal')">
        © 2019 Copyright: <a href="#">Vladimir Parakhin</a>
      </div>
      <b-modal style="direction: ltr" body-class="pl-0 pr-0 pt-0 pb-0" id="contactInfo-modal" centered hide-footer hide-header>
        <b-card no-body class="overflow-hidden" style="max-width: 540px;direction: ltr">
          <div class="row">
            <div class="col pr-0 mr-0" style="background-color: #343a40">

              <img src="../assets/logo_transparent.png"
                   height="275"
                   width="275"/>
            </div>
            <div class="col pl-0 pr-0">
              <b-card header-text-variant="white" header-bg-variant="dark" body-border-variant="no" header="Contact Info" class="">
                <b-card-text class="mt-5 ml-5">
                  <font-awesome-icon :icon="['fab', 'github']" class="mr-1"></font-awesome-icon>
                  <a href="https://github.com/Vladimir-pa/">Github</a>
                </b-card-text>
                <b-card-text class="ml-5">
                  <font-awesome-icon :icon="['fab', 'linkedin']" class="mr-1"> </font-awesome-icon>
                  <a href="https://www.linkedin.com/in/vladimir-parakhin/">LinkedIn</a>
                </b-card-text >
                <b-card-text class="ml-5 mb-3 pb-3">
                  <font-awesome-icon icon="envelope" class="mr-1"> </font-awesome-icon>
                  <a href="vov4ikpa@gmail.com">Email </a>
                </b-card-text>
              </b-card>
            </div>
          </div>
        </b-card>
      </b-modal>

    </b-navbar>
  </div>
  <!--    <nav class="navbar fixed-bottom py-3 bg-dark text-white-50 ">-->
  <!--        <div class="mx-auto">-->
  <!--            © 2019 Copyright:<a href="https://github.com/Vladimir-pa/">Vladimir Parakhin</a>-->
  <!--        </div>-->
  <!--    </nav>-->
</template>
<script>
    export default {
        name: 'dp-footer'
    }
</script>
