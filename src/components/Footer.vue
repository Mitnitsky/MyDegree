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
      <my-info-card/>

    </b-navbar>
  </div>
  <!--    <nav class="navbar fixed-bottom py-3 bg-dark text-white-50 ">-->
  <!--        <div class="mx-auto">-->
  <!--            © 2019 Copyright:<a href="https://github.com/Vladimir-pa/">Vladimir Parakhin</a>-->
  <!--        </div>-->
  <!--    </nav>-->
</template>
<script>
    import MyInfoCard from "./MyInfoCard";

    export default {
        name: 'dp-footer',
        components: {MyInfoCard}
    }
</script>
