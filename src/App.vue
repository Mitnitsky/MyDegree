<template>
  <el-config-provider :locale="locale">
    <div
      id="app"
      style="
        direction: rtl !important;
        font-family: Alef, Roboto, Helvetica, Arial, sans-serif !important;
        min-width: 965px !important;
      "
    >
      <el-container>
        <el-header style="padding: 0">
          <header-nav-bar />
        </el-header>
        <el-main>
          <div class="container-fluid" style="justify-content: center !important;">
            <!--            <el-row>-->
            <!--              <el-col span="6" offset="6">-->
            <!--                <autocomplete-->
            <!--                  @input="getItems"-->
            <!--                  @onSelect="selectedCourse"-->
            <!--                  :display-item="displayCourse"-->
            <!--                  :use-html-for-results="true"-->
            <!--                  :results="options"-->
            <!--                ></autocomplete>-->
            <!--              </el-col>-->
            <!--              <el-col v-if="courseSelected">-->
            <!--                <em>{{ course.full_name }} - </em>-->
            <!--                <em>{{ course.points }} </em>-->
            <!--              </el-col>-->
            <!--            </el-row>-->
            <semesters-tab-view style="margin: 5px" />
            <!--      <degree-summary />-->
          </div>
        </el-main>
        <el-footer class="footer-fixed-bottom">
          <dp-footer />
        </el-footer>
      </el-container>
    </div>
  </el-config-provider>
</template>
<script lang="ts">
import { defineComponent, reactive, ref } from "vue";
import { ElConfigProvider } from "element-plus";

import He from "element-plus/lib/locale/lang/he";
import HeaderNavBar from "@/components/Header.vue";
import { JsonCourse } from "@/store/classes/json_course_db";
import DpFooter from "@/components/Footer.vue";
// import DegreeSummary from "@/components/DegreeSummary.vue";
import SemestersTabView from "@/components/SemestersTabView.vue";
export default defineComponent({
  name: "App",
  components: {
    ElConfigProvider,
    HeaderNavBar,
    // DegreeSummary,
    DpFooter,
    SemestersTabView,
  },
  setup() {
    let json_courses;
    let c: JsonCourse = {
      full_name: "",
      name: "",
      number: "",
      points: 0,
      prerequisites: [],
      linked: [],
      identical: [],
      overlapping: [],
      inclusive: [],
      including: [],
      followed_by: [],
    };
    const course = reactive(c);
    const options = ref([]);
    const textPart = ref("");
    const courseSelected = ref(false);
    if (localStorage.getItem("courses")) {
      json_courses =
        typeof localStorage.getItem("courses") === "object"
          ? localStorage.getItem("courses")
          : JSON.parse(localStorage.getItem("courses") ?? "");
      if (!json_courses.version || json_courses.version < 7.0) {
        json_courses = require("./data/courses.json");
        localStorage.setItem("courses", JSON.stringify(json_courses));
      }
    } else {
      json_courses = require("./data/courses.json");
      localStorage.setItem("courses", JSON.stringify(json_courses));
    }
    const getItems = (text) => {
      textPart.value = text;
      if (text.length < 2) {
        options.value = [];
        return;
      }
      options.value = json_courses.courses.filter((course) =>
        course.full_name.includes(text)
      );
    };
    const displayCourse = (course) => {
      let start_of_highlight_index = course.full_name.indexOf(textPart.value);
      let end_of_highlight_index =
        start_of_highlight_index + textPart.value.length;
      return (
        "<span>" +
        course.full_name.substring(0, start_of_highlight_index) +
        "</span>" +
        "<mark>" +
        course.full_name.substring(
          start_of_highlight_index,
          end_of_highlight_index
        ) +
        "</mark>" +
        "<span>" +
        course.full_name.substring(end_of_highlight_index) +
        "</span>"
      );
    };
    const selectedCourse = (c: JsonCourse) => {
      courseSelected.value = true;
      course.full_name = c.full_name;
      course.name = c.name;
      course.number = c.number;
      course.points = c.points;
      course.prerequisites = c.prerequisites;
      course.linked = c.linked;
      course.identical = c.identical;
      course.overlapping = c.overlapping;
      course.inclusive = c.inclusive;
      course.including = c.including;
      course.followed_by = c.followed_by;
    };
    return {
      course,
      courseSelected,
      selectedCourse,
      displayCourse,
      json_courses,
      getItems,
      options,
      locale: He,
    };
  },
});
// eslint-disable-next-line @typescript-eslint/no-var-requires,@typescript-eslint/no-unused-vars
const rtlcss = require("rtlcss");
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
.footer-fixed-bottom {
  position: fixed;
  right: 0;
  left: 0;
  z-index: 1030;
  bottom: 0;
  margin-bottom: 0;
  border-width: 1px 0 0;
  padding: 0 !important;
  max-height: 40px;
}
.dropdown-item {
  text-align: right !important;
}

body,
html {
  direction: ltr;
}
</style>
