<template>
  <div>
    <b-navbar fixed="bottom"
              type="dark"
              variant="dark">
      <div @click="$bvModal.show('contactInfo-modal')"
           class="mx-auto"
           style="color: lightgray">
        © 2019 Copyright: <a href="#">Vladimir Parakhin</a>
      </div>
      <my-info-card/>
    </b-navbar>
  </div>
</template>
<script>
    import MyInfoCard from "./MyInfoCard";

    export default {
        name: 'dp-footer',
        components: {MyInfoCard}
    }
</script>
