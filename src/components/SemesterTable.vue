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
          <!--          <search-course-dialog />-->
        </modal>
      </b-button-group>
    </div>
  </div>
</template>

<script lang="ts">
import SemesterTableRow from "@/components/SemesterTableRow.vue";
import SemesterHeader from "@/components/SemesterTableHeader.vue";
import { defineComponent, ref } from "vue";
import { useStore } from "@/use/useStore";
import { USER_STORE } from "@/store/constants";
import { ElMessage, ElMessageBox } from "element-plus/es";
export default defineComponent({
  name: "SemesterTable",
  components: {  SemesterHeader, SemesterTableRow },
  props: {
    semester: {
      type: Object,
      default: () => {
        return { courses: [] };
      },
    },
  },
  setup(props) {
    const store = useStore();
    const searchDialogShown = ref(false);
    const moveFunction = (index, direction: "up" | "down") => {
      if (
        !(
          (index === props.semester.courses.length - 1 &&
            direction === "down") ||
          (index === 0 && direction === "up")
        )
      ) {
        if (direction === "up") {
          store.commit(USER_STORE.MUTATIONS.swapCourses, {
            a: index - 1,
            b: index,
          });
        } else if (direction === "down") {
          store.commit(USER_STORE.MUTATIONS.swapCourses, {
            a: index + 1,
            b: index,
          });
        }
      }
      store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const addRow = () => {
      store.commit(USER_STORE.MUTATIONS.addCourse);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const removeLastRow = () => {
      if (props.semester.courses.length > 0) {
        ElMessageBox.confirm("למחוק שורה בעלת תוכן?", {
          confirmButtonText: "כן",
          cancelButtonText: "לא",
          type: "warning",
          icon: "none",
        })
          .then(() => {
            store.commit(USER_STORE.MUTATIONS.removeLastRow);
            store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
            store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
            ElMessage({
              type: "success",
              message: "קורס נמחק בהצלחה",
            });
          })
          .catch(() => {
            ElMessage({
              type: "info",
              message: "המחיקה בוטלה",
            });
          });
      }
    };
    return {
      searchDialogShown,
      moveFunction,
      addRow,
      removeLastRow,
      headerTextVariant: "light",
      headerBgVariant: "dark",
      alignment: "flex-end",
    };
  },
});
</script>

<style>
.dropdown-toggle::after {
  display: none !important;
}

.dropdown-toggle::before {
  display: none !important;
}
</style>
