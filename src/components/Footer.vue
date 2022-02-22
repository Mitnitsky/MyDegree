<template>
  <div
    style="
      height: 100%;
      vertical-align: center;
      background-color: #545c64;
      color: #fff;
      justify-content: center;
      text-align: center;
    "
  >
    <span
      @click="infoCardVisible = true"
      style="
        text-align: center;
        position: relative;
        top: 25%;
        -ms-transform: translateY(-25%);
        -webkit-transform: translateY(-25%);
        transform: translateY(-25%);
      "
    >
      © Created by:
      <a @click="infoCardVisible = true" href="#" style="color: dodgerblue"
        >Vladimir Mitnitsky</a
      >
    </span>
    <my-info-card :visible="infoCardVisible" :set-visibility="setVisibility" />
  </div>
</template>
<script lang="ts">
import MyInfoCard from "./MyInfoCard.vue";
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "DpFooter",
  components: { MyInfoCard },

  setup() {
    const infoCardVisible = ref(false);
    const setVisibility = (status: boolean) => {
      infoCardVisible.value = status;
    };
    return {
      infoCardVisible,
      setVisibility,
    };
  },
});
</script>
