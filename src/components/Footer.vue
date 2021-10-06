<template>
  <div>
    <b-navbar fixed="bottom" type="dark" variant="dark">
      <div
        class="mx-auto"
        style="color: lightgray"
        @click="$bvModal.show('contactInfo-modal')"
      >
        © Created by: <a href="#">Vladimir Mitnitsky</a>
      </div>
      <my-info-card />
    </b-navbar>
  </div>
</template>
<script>
import MyInfoCard from "./MyInfoCard";

export default {
  name: "DpFooter",
  components: { MyInfoCard },
};
</script>
