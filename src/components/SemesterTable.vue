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
      class="d-flex justify-content-center"
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
        <b-modal
          v-model="showSearchModal"
          size="lg"
          hide-footer
          hide-header
          scrollable
          body-class="p-0"
          content-class="border-0"
        >
          <search-course-dialog @close="showSearchModal = false" />
        </b-modal>
      </b-button-group>
    </div>
  </div>
</template>

<script>
import SemesterTableRow from "@/components/SemesterTableRow";
import SemesterHeader from "@/components/SemesterTableHeader";
import SearchCourseDialog from "./SearchCourseDialog";

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
      showSearchModal: false,
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
        // Uses store mutation to swap courses (avoiding direct prop mutation)
        this.$store.commit("moveCourse", { index, direction });
      }
      this.$store.commit("reCalcCurrentSemester");
      this.$store.dispatch("updateSemesterAsync");
    },
    showModal() {
      this.showSearchModal = true;
    },
    addRow() {
      this.$store.commit("addCourse");
      this.$store.dispatch("updateSemesterAsync");
    },
    removeLastRow() {
      if (this.semester.courses.length > 0) {
        if (window.confirm("למחוק שורה בעלת תוכן?")) {
          this.$store.commit("removeLastRow");
          this.$store.commit("reCalcCurrentSemester");
          this.$store.dispatch("updateSemesterAsync");
        }
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
