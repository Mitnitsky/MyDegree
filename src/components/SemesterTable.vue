<template>
  <el-table :data="semester.courses">
    <el-table-column
      header-align="center"
      prop="category"
      label="קטגוריה"
      min-width="135"
    >
      <template #default="scope">
        <select
          v-model.number.lazy="scope.row.type"
          class="form-control courseType"
          :style="{ backgroundColor: choose_colors[scope.row.type % 10] }"
          @change.stop="updateField('type')"
        >
          <template v-for="(type, i) in course_types" :key="i">
            <option :style="{ backgroundColor: choose_colors[i] }" :value="i">
              {{ type.name }}
            </option>
          </template>
        </select>
      </template>
    </el-table-column>
    <el-table-column
      header-align="center"
      prop="number"
      label="מספר קורס"
      sortable
      min-width="90"
    >
      <template #default="scope">
        <el-input
          v-model.number.lazy="scope.row.number"
          class="form-control courseNumber"
          max="9999999"
          min="0"
          step="1"
          type="number"
          style=""
          @change="updateField('number', scope.row.number, scope.$index)"
        />
      </template>
    </el-table-column>
    <el-table-column
      header-align="center"
      prop="name"
      label="שם קורס"
      sortable
      min-width="250"
    >
      <template #default="scope">
        <el-input
          v-model.lazy="scope.row.name"
          class="form-control courseName"
          type="text"
          @change="updateField('name', scope.row.name, scope.$index)"
        />
      </template>
    </el-table-column>
    <el-table-column
      header-align="center"
      prop="points"
      label="נקודות"
      sortable
      :min-width="60"
    >
      <template #default="scope">
        <el-input
          v-model.number.lazy="scope.row.points"
          :class="[scope.row.points >= 0 ? '' : InputIsWrong]"
          class="form-control coursePoints"
          max="500"
          min="0"
          step="0.5"
          type="number"
          @change="updateField('points', scope.row.points, scope.$index)"
        />
      </template>
    </el-table-column>
    <el-table-column
      header-align="center"
      prop="grade"
      label="ציון"
      sortable
      :min-width="60"
    >
      <template #default="scope">
        <el-input
          v-if="scope.row.binary === false || scope.row.binary === undefined"
          v-model.number.lazy="scope.row.grade"
          :class="[
            scope.row.grade >= 0 && scope.row.grade <= 100 ? '' : InputIsWrong,
          ]"
          class="form-control courseGrade"
          max="100"
          min="0"
          step="1"
          type="number"
          @change="updateField('grade', scope.row.grade, scope.$index)"
        />
        <el-input
          v-else
          model-value="✔"
          :readonly="true"
          style="color: green; cursor: default"
          class="form-control courseGrade"
        />
      </template>
    </el-table-column>
    <el-table-column header-align="center" :min-width="45">
      <template #default="scope">
        <el-dropdown @command="handleRowCommand" placement="bottom-end">
          <el-button
            ><font-awesome-icon icon="ellipsis-v" size="sm"
          /></el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item
                :command="{
                  name: 'histogram',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
              >
                <font-awesome-icon
                  icon="chart-bar"
                  size="sm"
                  style="color: dodgerblue; margin-left: 5px"
                />
                הצג היסטוגרמות</el-dropdown-item
              >
              <el-dropdown-item
                :command="{
                  name: 'binary-status-update',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
              >
                <font-awesome-icon
                  :icon="scope.row.binary ? 'ban' : 'check'"
                  size="sm"
                  :style="{
                    visibility: scope.row.binary,
                    color: scope.row.binary ? 'red' : 'green',
                    marginLeft: '5px',
                  }"
                />
                {{ scope.row.binary ? "בטל" : "סמן" }} עובר בינארי
              </el-dropdown-item>
              <el-dropdown-item
                divided
                :command="{
                  name: 'clean-row',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
              >
                <font-awesome-icon
                  icon="broom"
                  size="sm"
                  style="color: burlywood; margin-left: 5px"
                />
                נקה שורה</el-dropdown-item
              >
              <el-dropdown-item
                :command="{
                  name: 'remove-row',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
              >
                <font-awesome-icon
                  icon="trash"
                  size="sm"
                  style="color: darkred; margin-left: 10px"
                />
                הסר שורה
              </el-dropdown-item>
              <el-dropdown-item
                :command="{
                  name: 'moveSemester',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
                v-if="semestersNumber > 1"
                divided
              >
                <font-awesome-icon
                  icon="share-square"
                  size="sm"
                  style="color: black; margin-left: 10px"
                />
                העבר סמסטר</el-dropdown-item
              >

              <el-dropdown-item
                :command="{
                  name: 'move-up',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
                :disabled="scope.$index === 0"
                :divided="semestersNumber === 1"
              >
                <font-awesome-icon
                  icon="arrow-up"
                  size="sm"
                  style="color: black; margin-left: 10px"
                />
                העלה</el-dropdown-item
              >

              <el-dropdown-item
                :command="{
                  name: 'move-down',
                  index: scope.$index,
                  status: !scope.row.binary,
                }"
                :disabled="scope.$index === semester.courses.length - 1"
              >
                <font-awesome-icon
                  icon="arrow-down"
                  size="sm"
                  style="color: black; margin-left: 10px"
                />
                הורד</el-dropdown-item
              >
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </template>
    </el-table-column>
  </el-table>
</template>

<script lang="ts">
import { computed, defineComponent, Ref, ref } from "vue";
import { useStore } from "@/use/useStore";
import { USER_STORE } from "@/store/constants";
import { ElMessage, ElMessageBox } from "element-plus/es";
import { CourseType } from "@/store/classes/course_types";
import $ from "jquery";
import { convertJsonToProperSelectBoxFormat } from "@/store/extensions/histogramFunctions";
import { HistogramObject, Option } from "@/store/classes/histogramObject";
import { Semester } from "@/store/classes/semester";
export default defineComponent({
  name: "SemesterTable",
  props: {
    semester: {
      type: Semester,
      default: () => {
        return { courses: [] };
      },
    },
  },
  setup(props) {
    const store = useStore();
    const selected_semester_grade_stats: Ref<Option[]> = ref([]);
    const histogram_img_link = ref("");
    const course_info: Ref<HistogramObject[]> = ref([]);
    const histogramVisibilityState = ref(false);
    const fields = [
      {
        key: "students",
        label: "סטודנטים",
      },
      {
        key: "passFail",
        label: "נכשל/עובר",
      },
      {
        key: "passPercent",
        label: "אחוז עוברים",
      },
      {
        key: "min",
        label: "ציון מינימלי",
      },
      {
        key: "max",
        label: "ציון מקסימלי",
      },
      {
        key: "average",
        label: "ממוצע",
      },
      {
        key: "median",
        label: "חציון",
      },
    ];
    let forceUpdate = ref(() => {
      console.log("");
    });
    const searchDialogShown = ref(false);
    const course_types = computed<CourseType[]>(() => {
      return store.getters[USER_STORE.GETTERS.COURSE_TYPES];
    });
    const semestersNumber = computed(() => {
      return store.getters[USER_STORE.GETTERS.SEMESTERS].length;
    });
    const activeSemestersNumber = computed(() => {
      return store.getters[USER_STORE.GETTERS.ACTIVE_SEMESTER];
    });
    const clearRow = (row_index) => {
      store.commit(USER_STORE.MUTATIONS.clearCourse, row_index);
      store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const deleteRow = (row_index) => {
      store.commit(USER_STORE.MUTATIONS.removeCourse, row_index);
      store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const showHistorgram = (course_index) => {
      let course_number = props.semester?.courses[course_index].number;
      $.getJSON(
        `https://michael-maltsev.github.io/technion-histograms/${course_number}/index.json`,
        (doc) => {
          course_info.value = convertJsonToProperSelectBoxFormat(doc).sort(
            function (a, b) {
              return +b.semester_number - +a.semester_number;
            }
          );
          if (course_info.value.length > 0) {
            selected_semester_grade_stats.value =
              course_info.value[0].options[0].value;
            updateURL(selected_semester_grade_stats.value, course_number);
          }
        }
      );
      histogramVisibilityState.value = true;
    };
    const hideHistogram = () => {
      histogramVisibilityState.value = false;
    };
    const updateURL = (options: Option[], course_number: string) => {
      let option = options[0];
      histogram_img_link.value = `https://michael-maltsev.github.io/technion-histograms/${course_number}/${option.semester_number}/${option.entry_name}.png`;
    };
    const moveToSemester = (course_index) => {
      store.commit(USER_STORE.MUTATIONS.moveCourseToSemester, {
        semester_index: activeSemestersNumber,
        course_index: course_index,
      });
      store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const handleRowCommand = (command: {
      name:
        | "binary-status-update"
        | "histogram"
        | "clean-row"
        | "remove-row"
        | "moveSemester"
        | "move-up"
        | "move-down";
      index: string;
      status: boolean;
    }) => {
      console.log(command);
      switch (command.name) {
        case "binary-status-update":
          updateField("binary", command.status, command.index);
          forceUpdate.value();
          break;
        case "histogram":
          showHistorgram(command.index);
          break;
        case "clean-row":
          clearRow(command.index);
          break;
        case "remove-row":
          deleteRow(command.index);
          break;
        case "moveSemester":
          moveToSemester(command.index);
          break;
        case "move-up":
          moveFunction(command.index, "up");
          break;
        case "move-down":
          moveFunction(command.index, "down");
          break;
      }
    };
    const updateField = (field, value, index) => {
      console.log(field, value, index);
      if (field)
        store.commit(USER_STORE.MUTATIONS.updateCourse, {
          field,
          value,
          index,
        });
      store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
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
      histogramVisibilityState,
      clearRow,
      deleteRow,
      showHistorgram,
      hideHistogram,
      moveToSemester,
      handleRowCommand,
      semestersNumber,
      updateField,
      searchDialogShown,
      moveFunction,
      addRow,
      removeLastRow,
      headerTextVariant: "light",
      headerBgVariant: "dark",
      alignment: "flex-end",
      course_types,
      f: forceUpdate,
      InputIsWrong: "inputIsWrong",
      choose_colors: [
        "white",
        "lightgreen",
        "lightpink",
        "lightblue",
        "lightgoldenrodyellow",
        "lightcyan",
        "lightsteelblue",
        "lavender",
        "plum",
        "#f2b4ba",
      ],
    };
  },
  mounted() {
    this.f = () => {
      console.log("cheburek");
      this.$forceUpdate();
    };
  },
});
</script>

<style>
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
.inputIsWrong {
  border-color: red !important;
}
.dropdown-toggle::after {
  display: none !important;
}

.dropdown-toggle::before {
  display: none !important;
}
.courseType {
  direction: rtl !important;
  display: block;
  width: 100%;
  height: calc(1.5em + 0.75rem + 2px);
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  font-weight: 400;
  line-height: 1.5;
  color: #495057;
  background-color: #fff;
  background-clip: padding-box;
  border: 1px solid #ced4da;
  border-radius: 0.25rem;
  transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
}

div.courseNumber > input {
  text-align: center !important;
  direction: ltr !important;
}

div.courseName > input {
  width: 99% !important;
  text-align: center !important;
  direction: rtl !important;
}
.form-control {
  width: 90%;
}
.courseGrade {
  text-align: center !important;
  direction: ltr !important;
}
div.courseGrade > input {
  text-align: center !important;
  direction: ltr !important;
}

div.coursePoints > input {
  text-align: center !important;
  direction: ltr !important;
}

th {
  width: 90%;
  color: #495057;
  background-color: #e9ecef;
  border-color: #dee2e6;
  text-align: center;
}

span.el-input__suffix {
  right: unset !important;
  left: 12px !important;
  top: unset !important;
}
.clickAbleHeader:hover {
  color: cornflowerblue;
  text-decoration: underline;
  cursor: pointer;
}
.el-table .el-table__cell {
  padding: 2px 0;
  min-width: 0;
  box-sizing: border-box;
  text-overflow: ellipsis;
  vertical-align: middle;
  position: relative;
  text-align: left;
  z-index: 1;
}
.el-table {
  --el-table-border-color: white !important;
  --el-table-row-hover-bg-color: white !important;
}
</style>
