<template>
  <div>
    <div class="row">
      <el-table :data="semester.courses" >
        <el-table-column header-align="center" prop="category" label="קטגוריה" min-width="135">
          <template #default="scope">
            <el-select
              v-model.number.lazy="scope.row.type"
              class="form-control courseType"
              :style="{
                width: '100%',
                backgroundColor: choose_colors[scope.row.type % 10],
              }"
              @change.stop="updateField('type', scope.row.type, scope.$index)"
            >
              <el-option
                :style="{ backgroundColor: choose_colors[index_2] }"
                v-for="(type, index_2) in course_types" :key="index_2" :label="type.name" :value="type.name"
              />
            </el-select>
          </template>
        </el-table-column>
        <el-table-column
          header-align="center"
          prop="number"
          label="מספר קורס"
          sortable
          min-width="135"
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
        <el-table-column header-align="center" prop="name" label="שם קורס" sortable min-width="360">
          <template #default="scope">
            <el-input
              v-model.lazy="scope.row.name"
              class="form-control courseName"
              type="text"
              @change="updateField('name', scope.row.name, scope.$index)"
            />
          </template>
        </el-table-column>
        <el-table-column header-align="center" prop="points" label="נקודות" sortable min-width="108">
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
        <el-table-column header-align="center" prop="grade" label="ציון" sortable min-width="108">
          <template #default="scope">
            <el-input
              v-if="
                scope.row.binary === false || scope.row.binary === undefined
              "
              v-model.number.lazy="scope.row.grade"
              :class="[
                scope.row.grade >= 0 && scope.row.grade <= 100
                  ? ''
                  : InputIsWrong,
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
              value="✔"
              readonly
              style="color: green; cursor: default"
              class="form-control courseGrade"
            />
          </template>
        </el-table-column>
        <el-table-column header-align="center" min-width="55">
          <template #default="scope">
            <el-dropdown placement="bottom-end">
              <el-button
                ><font-awesome-icon icon="ellipsis-v" size="sm"
              /></el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="showHistorgram">
                    <font-awesome-icon
                      icon="chart-bar"
                      size="sm"
                      style="color: dodgerblue; margin-left: 5px"
                    />
                    הצג היסטוגרמות</el-dropdown-item
                  >
                  <el-dropdown-item
                    v-if="!scope.row.binary"
                    @click="setCourseBinaryState(true)"
                  >
                    <font-awesome-icon
                      icon="check"
                      size="sm"
                      style="color: green; margin-left: 5px"
                    />
                    סמן עובר בינארי
                  </el-dropdown-item>
                  <el-dropdown-item v-else @click="setCourseBinaryState(false)">
                    <font-awesome-icon
                      icon="ban"
                      size="sm"
                      style="color: red; margin-left: 5px"
                    />
                    בטל עובר בינארי
                  </el-dropdown-item>
                  <el-dropdown-item divided @click="clearRow()">
                    <font-awesome-icon
                      icon="broom"
                      size="sm"
                      style="color: burlywood; margin-left: 5px"
                    />
                    נקה שורה</el-dropdown-item
                  >
                  <el-dropdown-item @click="deleteRow">
                    <font-awesome-icon
                      icon="trash"
                      size="sm"
                      style="color: darkred; margin-left: 10px"
                    />
                    הסר שורה
                  </el-dropdown-item>
                  <el-dropdown-item
                    @click="console.log('not implemented')"
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
                    @click="moveCourseInner('up')"
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
                    @click="moveCourseInner('down')"
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
import { computed, defineComponent, ref } from "vue";
import { useStore } from "@/use/useStore";
import { USER_STORE } from "@/store/constants";
import { ElMessage, ElMessageBox } from "element-plus/es";
export default defineComponent({
  name: "SemesterTable",
  // components: {  SemesterHeader, SemesterTableRow },
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
    const course_types = computed(() => {
      return store.getters[USER_STORE.GETTERS.COURSE_TYPES];
    });
    const semestersNumber = computed(() => {
      return store.getters[USER_STORE.GETTERS.SEMESTERS].length;
    });
    const updateField = (field, value, index) => {
      console.log(field, value);
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
});
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
.dropdown-toggle::after {
  display: none !important;
}

.dropdown-toggle::before {
  display: none !important;
}
.courseType {
  text-align-last: center;
}

.courseNumber {
  text-align: center;
  direction: ltr;
}

.courseName {
  width: 99%;
  text-align: center;
  direction: rtl;
}
.form-control {
  width: 90%;
}
.courseGrade {
  text-align: center;
  direction: ltr;
}

.coursePoints {
  text-align: center;
  direction: ltr;
}

th {
  width: 90%;
  color: #495057;
  background-color: #e9ecef;
  border-color: #dee2e6;
  text-align: center;
}

.clickAbleHeader:hover {
  color: cornflowerblue;
  text-decoration: underline;
  cursor: pointer;
}
</style>
