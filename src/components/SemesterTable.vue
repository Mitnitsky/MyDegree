<template>
  <div>
    <div class="row">
      <table class="table table-sm table-borderless" style="margin-right: 5px">
        <semester-header />
        <tbody>
          <semester-table-row
            v-for="(course, index) in semester.courses"
            :key="course.name + course.number + index"
            :course="course"
            :index="index"
            :table-size="semester.courses.length"
            :move-function="moveFunction"
          />
        </tbody>
      </table>
    </div>
    <div
      class="row justify-content-md-center"
      style="justify-content: center !important"
    >
      <b-button-group class="mx-1" style="direction: ltr">
        <b-button
          variant="info"
          style="border-right: #0072ec solid 1px"
          @click="addRow"
        >
          הוספת שורה
        </b-button>
        <b-button
          variant="primary"
          style="border-left: #0072ec solid 1px"
          @click="showModal"
        >
          חיפוש קורסים
        </b-button>
        <modal
          :max-height="800"
          :min-height="380"
          width="800"
          height="auto"
          name="search"
          scrollable
        >
          <search-course-dialog />
        </modal>
      </b-button-group>
    </div>
  </div>
</template>

<script>
import SemesterTableRow from "@/components/SemesterTableRow";
import SemesterHeader from "@/components/SemesterTableHeader";
import SearchCourseDialog from "./SearchCourseDialog";
import Vue from "vue";

export default {
  name: "SemesterTable",
  components: { SemesterTableRow, SemesterHeader, SearchCourseDialog },
  props: {
    semester: {
      type: Object,
      default: function () {
        return { courses: [] };
      },
    },
  },
  data() {
    return {
      headerTextVariant: "light",
      headerBgVariant: "dark",
      alignment: "flex-end",
    };
  },
  methods: {
    moveFunction(index, direction) {
      if (
        !(
          (index === this.semester.courses.length - 1 &&
            direction === "down)") ||
          (index === 0 && direction === "up")
        )
      ) {
        if (direction === "up") {
          let temp = this.semester.courses[index - 1];
          Vue.set(
            this.semester.courses,
            index - 1,
            this.semester.courses[index]
          );
          Vue.set(this.semester.courses, index, temp);
        } else if (direction === "down") {
          let temp = this.semester.courses[index + 1];
          Vue.set(
            this.semester.courses,
            index + 1,
            this.semester.courses[index]
          );
          Vue.set(this.semester.courses, index, temp);
        }
      }
      this.$store.commit("reCalcCurrentSemester");
      this.$store.dispatch("updateSemesterAsync");
    },
    showModal() {
      this.$modal.show("search");
    },
    addRow() {
      this.$store.commit("addCourse");
      this.$store.dispatch("updateSemesterAsync");
    },
    removeLastRow() {
      if (this.semester.courses.length > 0) {
        this.$bvModal
          .msgBoxConfirm("למחוק שורה בעלת תוכן?", {
            title: "אזהרה",
            size: "sm",
            headerBgVariant: "dark",
            headerTextVariant: "white",
            buttonSize: "md",
            cancelDisabled: "true",
            okVariant: "danger",
            okTitle: "כן",
            cancelTitle: "לא",
            autoFocusButton: "ok",
            footerClass: "p-2",
            hideHeaderClose: true,
            centered: true,
          })
          .then((v) => {
            if (v === true) {
              this.$store.commit("removeLastRow");
              this.$store.commit("reCalcCurrentSemester");
              this.$store.dispatch("updateSemesterAsync");
            }
          });
      }
    },
  },
};
</script>

<style>
.dropdown-toggle::after {
  display: none !important;
}

.dropdown-toggle::before {
  display: none !important;
}
</style>
