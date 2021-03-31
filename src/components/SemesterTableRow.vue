<template>
  <tr>
    <td>
      <select
        v-model.number.lazy="course_copy.type"
        :select-on-tab="true"
        class="form-control courseType"
        :style="{ backgroundColor: choose_colors[course.type % 10]}"
        @change.stop="updateField('type')"
      >
        <template v-for="(type, index_2) in course_types">
          <option
            :key="index_2"
            :style="{ backgroundColor: choose_colors[index_2]}"
            :value="index_2"
          >
            {{ type.name }}
          </option>
        </template>
      </select>
    </td>
    <td style="min-width: 90px">
      <input
        v-model.number.lazy="course_copy.number"
        class="form-control courseNumber"
        max="9999999"
        min="0"
        step="1"
        type="number"
        @change="updateField('number')"
      >
    </td>
    <td style="min-width: 250px;padding-right: 0">
      <input
        v-model.lazy="course_copy.name"
        class="form-control courseName"
        type="text"
        @change="updateField('name')"
      >
    </td>
    <td style="min-width: 60px">
      <input
        v-model.number.lazy="course_copy.points"
        :class="[course.points >= 0 ? '' : InputIsWrong]"
        class="form-control coursePoints"
        max="500"
        min="0"
        step="0.5"
        type="number"
        @change="updateField('points')"
      >
    </td>
    <td style="min-width: 60px">
      <input
        v-if="course.binary === false || course.binary === undefined"
        v-model.number.lazy="course_copy.grade"
        :class="[course.grade >= 0 && course.grade <= 100 ? '' : InputIsWrong]"
        class="form-control courseGrade"
        max="100"
        min="0"
        step="1"
        type="number"
        @change="updateField('grade')"
      >
      <input
        v-else
        v-b-popover.hover.top="'עובר בינארי'"
        value="✔"
        readonly
        style="color: green;cursor: default;"
        class="form-control courseGrade"
      >
    </td>
    <td
      class="text-center"
      style="min-width: 45px"
    >
      <b-dropdown
        id="dropdown-1"
        v-b-tooltip.hover.v-secondary
        dropleft
        variant="outline-secondary"
      >
        <template #button-content>
          <font-awesome-icon
            icon="ellipsis-v"
            size="sm"
          />
        </template>
        <b-dropdown-item
          :disabled="!(course_copy.number && course_copy.number && course_copy.number > 0)"
          @click="showHistorgram"
        >
          <font-awesome-icon
            icon="chart-bar"
            size="sm"
            style="color: dodgerblue; margin-left: 5px;"
          />
          הצג היסטוגרמות
        </b-dropdown-item>

        <b-dropdown-item
          v-if="!course.binary || course.binary === undefined"
          @click="setCourseBinaryState(true)"
        >
          <font-awesome-icon
            icon="check"
            size="sm"
            style="color: green; margin-left: 5px;"
          />
          סמן עובר בינארי
        </b-dropdown-item>
        <b-dropdown-item
          v-else
          @click="setCourseBinaryState(false)"
        >
          <font-awesome-icon
            icon="ban"
            size="sm"
            style="color: red; margin-left: 5px;"
          />
          בטל עובר בינארי
        </b-dropdown-item>
        <b-dropdown-divider />
        <b-dropdown-item @click="clearRow">
          <font-awesome-icon
            icon="broom"
            size="sm"
            style="color: burlywood; margin-left: 5px;"
          />
          נקה שורה
        </b-dropdown-item>
        <b-dropdown-item @click="deleteRow">
          <font-awesome-icon
            icon="trash"
            size="sm"
            style="color: darkred; margin-left: 10px;"
          />
          הסר שורה
        </b-dropdown-item>
        <b-dropdown-divider />
        <b-dropdown-item
          v-if="$store.state.user.semesters.length > 1"
          @click="$bvModal.show('course-move-'+index)"
        >
          <font-awesome-icon
            icon="share-square"
            size="sm"
            style="color: black; margin-left: 10px;"
          />
          העבר סמסטר
        </b-dropdown-item>
        <b-dropdown-item
          :disabled="index === 0"
          @click="moveCourseInner('up')"
        >
          <font-awesome-icon
            icon="arrow-up"
            size="sm"
            style="color: black; margin-left: 10px;"
          />
          העלה
        </b-dropdown-item>
        <b-dropdown-item
          :disabled="index === tableSize - 1"
          @click="moveCourseInner('down')"
        >
          <font-awesome-icon
            icon="arrow-down"
            size="sm"
            style="color: black; margin-left: 10px;"
          />
          הורד
        </b-dropdown-item>
      </b-dropdown>
    </td>
    <b-modal
      :id="'course-move-'+index"
      centered
      header-bg-variant="dark"
      header-text-variant="light"
      hide-header-close
      hide-footer
      hide-backdrop
      :title="'העבר קורס ' + course_copy.name + ' אל סמסטר'"
    >
      <b-list-group
        v-for="(semester,i_index) in $store.state.user.semesters"
        :key="i_index"
      >
        <b-list-group-item
          :disabled="$store.state.user.active_semester === i_index"
          href="#"
          :class="{'align-items-start': true, 'justify-content-center': true, 'text-muted': $store.state.user.active_semester === i_index}"
          style="text-align: right"
          @click="moveToSemester(i_index)"
        >
          סמסטר {{ semester.name }}
        </b-list-group-item>
      </b-list-group>
    </b-modal>
    <b-modal
      :id="'histogram-'+index"
      centered
      hide-backdrop
      size="xl"
      header-bg-variant="dark"
      header-text-variant="light"
      hide-header-close
    >
      <template #modal-title>
        <tag class="row justify-content-center">
          <p>
            היסטוגרמות עבור
            <bold>{{ course_copy.name }}</bold>
          </p>
        </tag>
      </template>
      <template #modal-footer>
        <div
          class="row justify-content-center"
          style="width: 100%"
        >
          <b-button
            variant="primary"
            size="sm"
            style="width: 50%"
            @click="hideHistogram"
          >
            סגור
          </b-button>
        </div>
      </template>
      <b-card
        header-bg-variant="dark"
        header-text-variant="white"

        style="margin-bottom: 10px; direction: rtl !important"
      >
        <div
          v-if="course_info && course_info.length > 0"
          class="col"
        >
          <div class="row justify-content-center align-self">
            <p
              v-if="selected_semester_grade_stats"
              style="text-align: center"
            >
              <strong>
                {{ selected_semester_grade_stats[0].semester_name }}
              </strong>
              <br
                v-if="selected_semester_grade_stats[0].staff !== undefined"
              >
              <strong
                v-if="selected_semester_grade_stats[0].staff !== undefined"
              >
                {{ selected_semester_grade_stats[0].staff }}
              </strong>
            </p>
          </div>
          <div class="row justify-content-center">
            <b-form-select
              v-model="selected_semester_grade_stats"
              :options="course_info"
              style="width: 75%"
              class="mb-2"
              @change="updateURL($event)"
            />
          </div>
        </div>
        <div
          v-else
          class="row mt-2 mb-2 mr-2 ml-2"
        >
          <strong>אין היסטוגרמות זמינות</strong>
        </div>
        <div
          v-if="selected_semester_grade_stats"
          class="row justify-content-center mt-3 ml-2 mr-2"
        >
          <b-table
            v-if="selected_semester_grade_stats"
            bordered
            fixed
            small
            style="text-align: center"
            :items="selected_semester_grade_stats"
            :fields="fields"
            head-variant="Light"
          />
        </div>
        <div class="row justify-content-center">
          <b-img
            v-if="histogram_img_link"
            rounded="true"
            :src="histogram_img_link"
            class="mb-2"
            style="cursor: zoom-in"
            fluid
            @click="$bvModal.show('histogram-modal')"
          />
          <b-modal
            id="histogram-modal"
            centered
            size="lg"
            hide-footer
          >
            <b-img
              v-if="histogram_img_link"
              rounded="true"
              size="xl"
              :src="histogram_img_link"
              fluid-grow
            />
          </b-modal>
        </div>
      </b-card>
    </b-modal>
  </tr>
</template>
<script>
import {clearCourse} from "@/store/classes/course";
import {createHelpers} from "vuex-map-fields";
import $ from "jquery";
import {convertJsonToProperSelectBoxFormat} from "@/store/extensions/histogramFunctions";

const {mapFields} = createHelpers({
  getterType: "getUserField",
  mutationType: "updateUserField"
});

export default {
  name: "SemesterTableCourseRow",

  props:
      {
        course: {
          type: Object,
          required: true
        },
        index: {
          type: Number,
          required: true
        },
        moveFunction: {
          type: Function,
          required: true
        },
        tableSize: {
          type: Number,
          required: true
        }
      },
  data() {
    return {
      selected_semester_grade_stats: null,
      course_info: null,
      histogram_img_link: null,
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
      course_copy: this.course,
      InputIsWrong: "inputIsWrong",
      choose_colors: ["white", "lightgreen", "lightpink", "lightblue", "lightgoldenrodyellow", "lightcyan", "lightsteelblue", "lavender", "plum", "#f2b4ba"],
    };
  },
  computed: {
    ...mapFields(["course_types"]),
  },
  methods: {
    clearRow() {
      clearCourse(this.course_copy);
      this.$store.commit("reCalcCurrentSemester");
    },
    deleteRow() {
      this.$store.commit("removeCourse", this.index);
      this.$store.commit("reCalcCurrentSemester");
    },
    showHistorgram() {
      let self = this;
      let update = this.updateURL;
      $.getJSON(
          `https://michael-maltsev.github.io/technion-histograms/${this.course_copy.number}/index.json`,
          function (doc) {
            self.course_info = convertJsonToProperSelectBoxFormat(doc).sort(
                function (a, b) {
                  return b.semester_number - a.semester_number;
                }
            );
            if (self.course_info.length > 0) {
              self.selected_semester_grade_stats = self.course_info[0].options[0].value;
              update(self.selected_semester_grade_stats);
            }
          }
      );
      this.$bvModal.show("histogram-" + this.index);
    },
    hideHistogram() {
      this.$bvModal.hide("histogram-" + this.index);
    },
    updateURL(event) {
      let event_payload = event[0];
      this.histogram_img_link = `https://michael-maltsev.github.io/technion-histograms/${this.course_copy.number}/${event_payload.semester_number}/${event_payload.entry_name}.png`;
    },
    setCourseBinaryState(state) {
      this.course_copy.binary = state;
      this.updateField('binary');
      this.$forceUpdate();
    },
    updateField(field) {
      let value = this.course_copy[field];
      if (field)
        this.$store.commit("updateCourse", {field, value, index: this.index});
      this.$store.commit("reCalcCurrentSemester");
      this.$store.dispatch("updateSemesterAsync");
    },
    moveCourseInner(direction) {
      this.moveFunction(this.index, direction);
    },
    moveToSemester(semester_index){
      this.$store.commit("moveCourseToSemester", {semester_index: semester_index, course_index: this.index});
      this.$store.commit("reCalcCurrentSemester");
    }

  }
};
</script>

<style scoped>
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.inputIsWrong {
  border-color: red !important;
}

.courseType {
  text-align-last: center;
}

.courseNumber {
  text-align: center;
  direction: ltr;
}

.courseName {
  text-align: center;
  direction: rtl;
}

.courseGrade {
  text-align: center;
  direction: ltr;
}

.coursePoints {
  text-align: center;
  direction: ltr;
}

</style>
