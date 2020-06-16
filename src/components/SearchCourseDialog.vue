<template>
  <b-card dir="rtl" no-body style="min-height: 410px;">
    <div class="justify-content-center">
      <div
        class="row justify-content-between"
        style="background-color: #343a40;min-height: 50px"
      >
        <div class="col-auto">
          <h5 style="color: white;margin-top: 12px;margin-right: 10px;">
            חיפוש קורסים
          </h5>
        </div>
        <div class="col-auto mr-auto">
          <b-button
            style="border-color: #343a40;margin-top:6px;font-weight: bolder;margin-left: 2px"
            variant="outline-light"
            @click="hideSearchModal"
            >X
          </b-button>
        </div>
      </div>
      <div class="p-2">
        <autocomplete
          id="auto-input"
          aria-label="חיפוש קורסים"
          placeholder="חיפוש קורסים"
          style="text-align: right"
          :auto-select="true"
          :get-result-value="getResultValue"
          :search="search"
          @submit="courseChosen"
        />
        <b-card
          v-if="show"
          :header="selected_course.full_name"
          class="text-center"
          header-bg-variant="dark"
          header-text-variant="white"
          style="text-align: right;color: black;margin-top: 7px;min-height: 300px"
        >
          <b-card no-body style="margin-bottom: 10px; ">
            <template v-slot:header>
              <strong class="mb-0">נקודות</strong>
            </template>
            <p style="margin-top: 5px; margin-bottom: 10px">
              {{ selected_course.points }}
            </p>
          </b-card>

          <div class="row justify-content-center mb-2">
            <b-button v-if="show" type="primary" @click="addCourse">
              הוסף קורס
            </b-button>
          </div>
          <div class="row justify-content-center mb-2">
            <b-toast
              id="added-course"
              static
              variant="primary"
              auto-hide-delay="5000"
            >
              <div class="row" style="padding: 10px">
                <p style="font-size: larger">
                  קורס: "{{ selected_course.full_name }}" הוסף בהצלחה!
                </p>
              </div>
              <div class="row justify-content-center">
                <p
                  class="mr-1"
                  style="font-size: larger; font-weight: bold; text-decoration: underline;cursor: pointer;color: darkorange;"
                  @click="removeLastAddedCourse()"
                >
                  בטל הוספה
                </p>
              </div>
            </b-toast>
          </div>
          <b-button
            v-if="collapsedHistogram"
            style="margin: 5px;"
            variant="outline-secondary"
            @click="
              collapsedHistogram = !collapsedHistogram;
              collapseHistogram(true);
            "
            >הראה היסטוגרמות&Darr;
          </b-button>
          <b-button
            v-if="!collapsedHistogram"
            style="margin: 5px;"
            variant="secondary"
            @click="
              collapsedHistogram = !collapsedHistogram;
              collapseHistogram(false);
            "
            >הסתר היסטוגרמות &Uarr;
          </b-button>
          <b-button
            v-if="collapsedPrereq"
            style="margin: 5px;"
            variant="outline-secondary"
            @click="
              collapsedPrereq = !collapsedPrereq;
              collapsePrerequisites();
            "
            >הראה קורסי קדם/צמודים&Darr;
          </b-button>
          <b-button
            v-if="!collapsedPrereq"
            style="margin: 5px"
            variant="secondary"
            @click="
              collapsedPrereq = !collapsedPrereq;
              collapsePrerequisites();
            "
            >הראה קורסי קדם/צמודים&Uarr;
          </b-button>

          <b-button
            v-if="collapsedExtraInfo"
            v-b-popover.hover.top="'קורסים מוכלים/מכילים/ללא זיכוי נוסף'"
            style="margin: 5px;"
            variant="outline-secondary"
            @click="
              collapsedExtraInfo = !collapsedExtraInfo;
              collapseExtraInfo();
            "
            >הראה מידע נוסף &Darr;
          </b-button>
          <b-button
            v-if="!collapsedExtraInfo"
            style="margin: 5px"
            variant="secondary"
            @click="
              collapsedExtraInfo = !collapsedExtraInfo;
              collapseExtraInfo();
            "
            >הסתר מידע נוסף &Uarr;
          </b-button>
          <b-collapse id="collapse-histograms">
            <b-card
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px; "
            >
              <template v-slot:header>
                <strong class="mb-0">היסטוגרמות</strong>
              </template>
              <div v-if="course_info.length > 0" class="col mt-2 ">
                <p v-if="selected_semester_grade_stats">
                  <strong>{{
                    selected_semester_grade_stats[0].semester_name
                  }}</strong>
                  <br
                    v-if="selected_semester_grade_stats[0].staff !== undefined"
                  />
                  <strong
                    v-if="selected_semester_grade_stats[0].staff !== undefined"
                    >{{ selected_semester_grade_stats[0].staff }}</strong
                  >
                </p>
                <b-form-select
                  v-model="selected_semester_grade_stats"
                  :options="course_info"
                  class="mb-2"
                  @change="updateURL($event)"
                ></b-form-select>
              </div>
              <div v-else class="mt-2 mb-2 mr-2 ml-2">
                <strong>אין היסטוגרמות זמינות</strong>
              </div>
              <div v-if="selected_semester_grade_stats" class="mt-3 ml-2 mr-2">
                <b-table
                  v-if="selected_semester_grade_stats"
                  bordered
                  small
                  fixed
                  :items="selected_semester_grade_stats"
                  :fields="fields"
                  head-variant="Light"
                ></b-table>
                <b-img
                  v-if="histogram_img_link"
                  rounded
                  :src="histogram_img_link"
                  class="mb-2"
                  style="cursor: zoom-in"
                  fluid
                  @click="$bvModal.show('histogram-modal')"
                ></b-img>
                <b-modal id="histogram-modal" centered size="lg" hide-footer>
                  <b-img
                    v-if="histogram_img_link"
                    rounded
                    size="xl"
                    :src="histogram_img_link"
                    fluid-grow
                  ></b-img>
                </b-modal>
              </div>
            </b-card>
          </b-collapse>
          <b-collapse id="collapse-prereq-courses">
            <b-card
              v-if="selected_course.prerequisites[0].length > 0"
              v-model="selected_course.prerequisites"
              header="קורסי קדם"
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px"
            >
              <b-list-group
                v-for="(prerequisites, index) in selected_course.prerequisites"
                :key="index"
              >
                <b-list-group-item
                  v-for="(course, inner_index) in prerequisites"
                  :id="parseInt(index) + '_' + parseInt(inner_index) + '_preq'"
                  :key="inner_index"
                  :style="{ color: checkIfExists(course, 'prerequisite') }"
                  href="#"
                  @click="findPrerequisites($event)"
                  >{{ course }}
                  <b-popover
                    v-if="checkIfExists(course, 'prerequisite') === 'red'"
                    :target="
                      parseInt(index) + '_' + parseInt(inner_index) + '_preq'
                    "
                    placement="top"
                    triggers="hover"
                    ><span style="color: red">
                      קורס זה לא נמצא בתואר<br />
                      (עד סמסטר נוכחי לא כולל)
                    </span></b-popover
                  >
                </b-list-group-item>
                <p
                  v-if="index < selected_course.prerequisites.length - 1"
                  style="margin-bottom: 2px"
                >
                  או-
                </p>
              </b-list-group>
            </b-card>
            <b-card
              v-if="selected_course.linked.length > 0"
              v-model="selected_course.linked"
              header="קורסים צמודים"
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px"
            >
              <b-list-group style="margin-bottom: 7px;">
                <b-list-group-item
                  v-for="(linked, inner_index) in selected_course.linked"
                  :id="parseInt(index) + '_' + parseInt(inner_index) + '_link'"
                  :key="linked"
                  href="#"
                  :style="{ color: checkIfExists(linked, 'linked') }"
                  @click="findPrerequisites($event)"
                  >{{ linked }}
                  <b-popover
                    v-if="checkIfExists(linked, 'prerequisite') === 'red'"
                    :target="
                      parseInt(index) + '_' + parseInt(inner_index) + '_link'
                    "
                    placement="top"
                    triggers="hover"
                    variant="warning"
                    ><span style="">
                      קורס זה לא נמצא בתואר<br />
                      (עד סמסטר נוכחי כולל)
                    </span></b-popover
                  >
                </b-list-group-item>
              </b-list-group>
            </b-card>
          </b-collapse>
          <b-collapse id="collapse-additional-info">
            <b-card
              v-if="selected_course.overlapping.length > 0"
              v-model="selected_course.overlapping"
              header="קורסים ללא זיכוי נוסף"
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px"
            >
              <b-list-group style="margin-bottom: 7px;border-color: #005cbf">
                <b-list-group-item
                  v-for="overlapping in selected_course.overlapping"
                  :key="overlapping"
                  :style="{ color: checkIfExists(overlapping, 'other') }"
                  href="#"
                  @click="findPrerequisites($event)"
                  >{{ overlapping }}
                </b-list-group-item>
              </b-list-group>
            </b-card>
            <b-card
              v-if="selected_course.inclusive.length > 0"
              v-model="selected_course.inclusive"
              header="קורסים כלולים:"
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px"
            >
              <b-list-group style="margin-bottom: 7px; border-color: #005cbf">
                <b-list-group-item
                  v-for="inclusive in selected_course.inclusive"
                  :key="inclusive"
                  :style="{ color: checkIfExists(inclusive, 'other') }"
                  href="#"
                  @click="findPrerequisites($event)"
                  >{{ inclusive }}
                </b-list-group-item>
              </b-list-group>
            </b-card>
            <b-card
              v-if="selected_course.including.length > 0"
              v-model="selected_course.including"
              header="קורסים מכילים"
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px"
            >
              <b-list-group style="margin-bottom: 7px; border-color: #005cbf">
                <b-list-group-item
                  v-for="including in selected_course.including"
                  :key="including"
                  :style="{ color: checkIfExists(including, 'other') }"
                  href="#"
                  :v-b-popover="'Popover!'"
                  @click="findPrerequisites($event)"
                  >{{ including }}
                </b-list-group-item>
              </b-list-group>
            </b-card>
          </b-collapse>
        </b-card>
      </div>
    </div>
  </b-card>
</template>

<script>
import Autocomplete from "@trevoreyre/autocomplete-vue";
import { convertJsonToProperSelectBoxFormat } from "../store/aux/histogramFunctions";
import $ from "jquery";

let json_courses;

if (localStorage.getItem("courses")) {
  json_courses =
    typeof localStorage.getItem("courses") === "object"
      ? localStorage.getItem("courses")
      : JSON.parse(localStorage.getItem("courses"));
  if (!json_courses.version || json_courses.version <= 1.0) {
    json_courses = require("../data/courses.json");
    localStorage.setItem("courses", JSON.stringify(json_courses));
  }
} else {
  json_courses = require("../data/courses.json");
  localStorage.setItem("courses", JSON.stringify(json_courses));
}

export default {
  name: "SearchCourseDialog",
  components: {
    Autocomplete
  },

  data() {
    return {
      show: false,
      collapsedExtraInfo: true,
      collapsedPrereq: true,
      collapsedHistogram: true,
      grab: "grab",
      bgc: "transparent",
      selected_semester_grade_stats: null,
      course_info: null,
      last_added_course_index: null,
      fields: [
        {
          key: "students",
          label: "סטודנטים"
        },
        {
          key: "passFail",
          label: "נכשל/עובר"
        },
        {
          key: "passPercent",
          label: "אחוז עוברים"
        },
        {
          key: "min",
          label: "ציון מינימלי"
        },
        {
          key: "max",
          label: "ציון מקסימלי"
        },
        {
          key: "average",
          label: "ממוצע"
        },
        {
          key: "median",
          label: "חציון"
        }
      ],
      selected_course: {
        full_name: "",
        name: "",
        number: "",
        points: "",
        prerequisites: "",
        linked: "",
        overlapping: "",
        inclusive: "",
        including: ""
      },
      histogram_img_link: null,
      remove: json_courses,
      options: json_courses.courses
    };
  },
  methods: {
    hideSearchModal() {
      this.$modal.hide("search");
    },
    search(input) {
      if (input.length < 2) {
        return [];
      }
      return this.options.filter(e => e.full_name.includes(input));
    },
    getResultValue(result) {
      return result.full_name;
    },
    courseChosen(course) {
      this.show = true;
      this.selected_course = course;
      this.course_info = [];
      this.histogram_img_link = null;
      this.selected_semester_grade_stats = null;
      if (!this.collapsedHistogram) {
        this.collapseHistogram(false);
        this.collapsedHistogram = !this.collapsedHistogram;
      }
      if (!this.collapsedExtraInfo) {
        this.collapseExtraInfo(true);
        this.collapsedExtraInfo = !this.collapsedExtraInfo;
      }
      if (!this.collapsedPrereq) {
        this.collapsePrerequisites(true);
        this.collapsedPrereq = !this.collapsedPrereq;
      }
    },
    addCourse() {
      if (
        !(
          this.selected_course.name.includes("ספורט") ||
          this.selected_course.name.includes("גופני")
        )
      ) {
        let course_number_and_answer = {
          course_number: this.selected_course.number,
          answer: ""
        };
        this.$store.commit("checkIfCourseExists", course_number_and_answer);
        if (course_number_and_answer.answer !== false) {
          let message =
            "הקורס קיים בסמסטר " +
            course_number_and_answer.answer +
            ", להוסיף בכל זאת?";
          this.$bvModal
            .msgBoxConfirm(message, {
              title: "אזהרה",
              headerBgVariant: "dark",
              headerTextVariant: "white",
              size: "sm",
              buttonSize: "md",
              cancelDisabled: "true",
              okVariant: "danger",
              okTitle: "כן",
              autoFocusButton: "ok",
              cancelTitle: "לא",
              footerClass: "p-2",
              hideHeaderClose: true,
              centered: true
            })
            .then(v => {
              if (v === true) {
                let selected_course_and_added_index = {
                  course: this.selected_course,
                  added_index: this.last_added_course_index
                };
                this.$store.commit(
                  "addCourseWithDataReturningIndex",
                  selected_course_and_added_index
                );
                this.last_added_course_index =
                  selected_course_and_added_index.added_index;
                this.$store.commit("reCalcCurrentSemester");
                this.$bvToast.show("added-course");
              }
            });
        } else {
          let selected_course_and_added_index = {
            course: this.selected_course,
            added_index: this.last_added_course_index
          };
          this.$store.commit(
            "addCourseWithDataReturningIndex",
            selected_course_and_added_index
          );
          this.last_added_course_index =
            selected_course_and_added_index.added_index;
          this.$store.commit("reCalcCurrentSemester");
          this.$bvToast.show("added-course");
        }
      } else {
        let selected_course_and_added_index = {
          course: this.selected_course,
          added_index: this.last_added_course_index
        };
        this.$store.commit(
          "addCourseWithDataReturningIndex",
          selected_course_and_added_index
        );
        this.last_added_course_index =
          selected_course_and_added_index.added_index;
        this.$store.commit("reCalcCurrentSemester");
        this.$bvToast.show("added-course");
      }
    },
    removeLastAddedCourse() {
      this.$bvToast.hide("added-course");
      this.$store.commit("removeCourse", this.last_added_course_index);
    },
    findPrerequisites(event) {
      let course_name = event.target.innerText.split(":")[0];
      this.courseChosen(
        this.options.filter(course => {
          return course.full_name.includes(course_name);
        })[0]
      );
      window.console.log(this.selected_course);
    },
    collapseExtraInfo() {
      this.$root.$emit("bv::toggle::collapse", "collapse-additional-info");
    },
    collapsePrerequisites() {
      this.$root.$emit("bv::toggle::collapse", "collapse-prereq-courses");
    },
    collapseHistogram(fetch) {
      if (fetch) {
        let self = this;
        $.getJSON(
          `https://michael-maltsev.github.io/technion-histograms/${this.selected_course.number}/index.json`,
          function(doc) {
            self.course_info = convertJsonToProperSelectBoxFormat(doc).sort(
              function(a, b) {
                return b.semester_number - a.semester_number;
              }
            );
          }
        );
      }
      this.$root.$emit("bv::toggle::collapse", "collapse-histograms");
    },
    updateURL(event) {
      let event_payload = event[0];
      this.histogram_img_link = `https://michael-maltsev.github.io/technion-histograms/${this.selected_course.number}/${event_payload.semester_number}/${event_payload.entry_name}.png`;
    },
    checkIfExists(course_full_name, type) {
      let course_name = course_full_name.split(":")[1];
      if (course_name.includes("השלמות")) {
        return "black";
      }
      let course_number = course_full_name.split(":")[0];
      let course_number_answer_semester = {
        course_number: course_number,
        answer: "",
        semester: -1
      };
      if (type === "prerequisite") {
        this.$store.commit("checkPrerequisites", course_number_answer_semester);
      } else if (type === "linked") {
        this.$store.commit("checkLinear", course_number_answer_semester);
      } else {
        this.$store.commit(
          "checkIfCourseExists",
          course_number_answer_semester
        );
        //It's bad if one of inclusive/including/similar courses are in the table
        return course_number_answer_semester.answer === true ? "red" : "black";
      }
      return course_number_answer_semester.answer === false ? "red" : "black";
    }
  }
};
</script>
