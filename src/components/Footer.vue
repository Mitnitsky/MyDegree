<template>
  <div
    style="
      height: 100%;
      vertical-align: center;
      background-color: #343a40;
      color: #fff;
      justify-content: center;
      text-align: center;
    "
  >
    <span
      style="
        text-align: center;
        position: relative;
        top: 25%;
        -ms-transform: translateY(-25%);
        -webkit-transform: translateY(-25%);
        transform: translateY(-25%);
      "
      @click="infoCardVisible = true"
    >
      © Created by:
      <a href="#" style="color: dodgerblue" @click="infoCardVisible = true"
        >Vladimir Mitnitsky</a
      >
    </span>
    <my-info-card :set-visibility="setVisibility" :visible="infoCardVisible" />
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
