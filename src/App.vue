<template>
  <el-config-provider :locale="locale">
    <div
      id="app"
      style="
        direction: rtl !important;
        font-family: Alef, Roboto, Helvetica, Arial, sans-serif;
        min-width: 838px !important;
      "
    >
      <header-nav-bar />
      <div class="container-fluid">
        <!--      <semesters-tab-view style="margin: 5px" />-->
        <!--      <degree-summary />-->
        <!--      <dp-footer />-->
      </div>
    </div>
  </el-config-provider>
</template>
<script lang="ts">
import { defineComponent } from "vue";
import { ElConfigProvider } from "element-plus";
import He from "element-plus/es/locale/lang/he";

import HeaderNavBar from "@/components/Header.vue";
// import DpFooter from "@/components/Footer.vue";
// import DegreeSummary from "@/components/DegreeSummary.vue";
// import SemestersTabView from "@/components/SemestersTabView.vue";
export default defineComponent({
  name: "App",
  components: {
    ElConfigProvider,
    // DegreeSummary,
    // DpFooter,
    HeaderNavBar,
    // SemestersTabView,
  },
  setup() {
    return {
      locale: He,
    };
  },
});
// eslint-disable-next-line @typescript-eslint/no-var-requires
const rtlcss = require('rtlcss');
const result = rtlcss.process("body { direction:ltr; }");
</script>

<style>
@import "./fonts/Alef/stylesheet.css";

input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none !important;
  margin: 0;
}

/*//Firefox fix of input fields */
input[type="number"] {
  -moz-appearance: textfield; /* Firefox arrows on numeric fields */
}

@supports (-moz-appearance: none) {
  /*Firefox spin-box fix*/
  select {
    -moz-appearance: none !important;
    background: transparent
      url("data:image/gif;base64,R0lGODlhBgAGAKEDAFVVVX9/f9TU1CgmNyH5BAEKAAMALAAAAAAGAAYAAAIODA4hCDKWxlhNvmCnGwUAOw==")
      left center no-repeat !important;
    background-position: calc(5px) center !important;
  }
}

.dropdown-item {
  text-align: right !important;
}
body,
html {
  direction: ltr;
}
</style>
